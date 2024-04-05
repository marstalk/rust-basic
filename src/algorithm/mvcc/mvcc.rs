#![allow(dead_code)]

use std::{
    collections::HashMap,
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::algorithm::linkedlist::listnode_generic::LinkedList;
/// 原子性： undo log
/// 持久性： redo log
/// 隔离性： 锁 | MVCC
///

/// [Current Read]
/// select ... lock in share mode;  共享锁
/// select ... for update;  排他锁
/// insert ...; 排他锁
/// update ...; 排他锁， update ... where
/// delete ...; 排他锁,  delete ... where
/// transation isolation level: Serial
///
/// mvcc:  [Snapshot Read] use mvcc agorithm to return a reasonable version of data.
/// select * from;

/**
 * 每一行数据可能会有多个版本快照，这些快照构成了一个版本链。
 * 其实每个快照都是一个undo log。
 *
 * begin;  // 创建一个事务id，事务id全局递增
 * select ....; // 快照读
 * commit; // 提交事务
 */
pub struct Version {
    age: u32,

    // 自增id
    db_row_id: u32,
    // 创建或者修改本记录的最后一个事务id。
    db_trx_id: u32,
    // 回滚指针，指向这条记录的上一个版本的undo log记录，存储
    db_roll_ptr: Option<Box<Version>>,
}
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
            && self.db_row_id == other.db_row_id
            && self.db_trx_id == other.db_trx_id
            && self.db_roll_ptr == other.db_roll_ptr
    }
}
impl Eq for Version {}
impl Version {
    pub fn new(age: u32, db_row_id: u32, db_trx_id: u32) -> Version {
        Version {
            age,
            db_row_id,
            db_trx_id,
            db_roll_ptr: None,
        }
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Version: age={}, db_row_id={}, db_trx_id={}",
            self.age, self.db_row_id, self.db_trx_id
        )
    }
}
impl Clone for Version {
    fn clone(&self) -> Self {
        Version {
            age: self.age,
            db_row_id: self.db_row_id,
            db_trx_id: self.db_trx_id,
            db_roll_ptr: self.db_roll_ptr.clone(),
        }
    }
}

/**
 * 事务可见性算法实现：
 * 在某个事务中trx_id=x，执行select ... id=1的时候，会生成一个ReadView（即每个请求会有一个ReadView对象，这里有个细节：
 * 1. 在RC中，每次select都生成新的ReadView。
 * 2. 在RR中，多次select都使用第一次select的ReadView。
 * id=1这个一行的数据有多个版本（快照），该返回哪个？通过ReadView来判断
 *
 * 放在时间坐标轴是这样的：
 *     A             B            C
 * -----------|-------------|---------->
 *            |             |
 *        min_trx_id   max_trx_id
 *
 * 当一个事务读取某个数据时，会通过各个快照版本的db_trx_id跟坐标轴上的位置来进行可见性判断：
 * 遍历所有的版本，从最新到最旧：
 * if db_trx_id == creator_trx_id:
 *      当前事务在访问自己修改过的记录，则该版本可以被当前事务访问。返回当前快照。
 * else if db_trx_id < min_trx_id:
 *      当前事务能看见db_trx_id所在记录， 返回当前快照。
 * else if db_trx_id > max_trx_id:
 *      表示db_trx_id所在的记录在ReadView生成之后，那么对于当前事务肯定不可见， 返回当前快照。
 * else db_trx_id in trx_list: (注意，trx_list不一定时连续的事务id，有些事务可能已经提交了，提交之后，会从集合中删除)
 *      在readView生成时刻，这个事务还是在活跃状态，还没有commit，修改的数据当前事务也看不到，不能返回当前快照。
 * else:
 *      这个事务在readView生成之前就已经开始commit，那么修改的结果时可以看见事务的。continue
 *       
 *
 * 全局维护一个ReadView，在事务begin, commit, rollback操作时修改这个ReadView
 * begin:
 *      trx_id = global_trx_id +1;
 *      trx_list.put_last(trx_id);
 *      max_trx_id += 1;
 * commit:
 *      trx_list.remove_first(trx_id);
 *      min_trx_id = min(trx_list);
 * rollback:
 *      trx_list.remove_first(trx_id);
 *      min_trx_id = min(trx_list);
 */
pub struct ReadView {
    // 当前系统中所有的活跃事务id（全局的），活跃事务id是指开启了的，但未提交的事务的id
    trx_list: LinkedList<u32>,
    // 生成ReadView时，当前系统中活跃的读写事务中最小的id，即min(trx_list)
    min_trx_id: u32,
    // 当前系统中事务的id值最大的那个事务id值再+1，也就是系统中下一个要生成的事务id，注意并不是max(trx_id)
    // 因为trx_id的id有可能已经提交了。
    max_trx_id: u32,
    // 生成本ReadView的事务id。
    creator_trx_id: u32,
}

enum IsolationLevel {
    RU,
    RC,
    RR,
    Serial,
}

pub struct DB {
    // key is row_id
    version_map: HashMap<u32, LinkedList<Version>>,
    globla_tx_id: AtomicU32,
    // key is trx_id
    read_view_map: HashMap<u32, ReadView>,
    // TODO LinkedList thread safe?
    trx_list: LinkedList<u32>,
}

impl DB {
    pub fn new(init_trx_id: AtomicU32) -> DB {
        DB {
            globla_tx_id: init_trx_id,
            version_map: HashMap::new(),
            read_view_map: HashMap::new(),
            trx_list: LinkedList::new(),
        }
    }

    fn begin(&mut self) -> u32 {
        // get trx_id
        let trx_id = self.globla_tx_id.fetch_add(1, Ordering::SeqCst);
        // add to trx_list as active trx.
        self.trx_list.push_head(trx_id);
        trx_id
    }

    fn commit(&mut self, tx_id: u32) {
        // remove trx from trx_list.
        self.trx_list.remove_first_hit(tx_id);
    }

    fn select(&mut self, tx_id: u32, id: u32, isolation: IsolationLevel) -> Option<&Version> {
        // if readView is none then create one.
        // else if isolation is Rc then create one.

        match (isolation, self.read_view_map.get(&tx_id)) {
            (_, None) => {
                self.read_view_map.insert(
                    tx_id,
                    ReadView {
                        trx_list: self.trx_list.clone(),
                        min_trx_id: *self.trx_list.tail_value().unwrap(),
                        max_trx_id: self.globla_tx_id.load(Ordering::SeqCst) + 1,
                        creator_trx_id: tx_id,
                    },
                );
            }
            (IsolationLevel::RC, _) => {
                self.read_view_map.insert(
                    tx_id,
                    ReadView {
                        trx_list: self.trx_list.clone(),
                        min_trx_id: *self.trx_list.tail_value().unwrap(),
                        max_trx_id: self.globla_tx_id.load(Ordering::SeqCst) + 1,
                        creator_trx_id: tx_id,
                    },
                );
            }
            (_, _) => {}
        }

        let read_view = self.read_view_map.get(&tx_id).unwrap();
        let version = self.version_map.get(&id).unwrap();

        // TODO linkedList iterator
        // while let Some(version) = version.{
        //
        // }

        None
    }

    fn update(&mut self, tx_id: u32, id: u32, age: u32) {
        let a = self
            .version_map
            .entry(id)
            .or_insert(LinkedList::new())
            .push_head(Version::new(age, id, tx_id));
    }

    fn select_for_update(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_commit() {
        // age = 4000 at the begining.
        let mut db = DB::new(AtomicU32::new(200));
        let tx1 = db.begin(); // 201
        let tx2 = db.begin(); // 202
        let tx3 = db.begin(); // 203
        let tx4 = db.begin(); // 204
        let tx5 = db.begin(); // 205

        db.update(tx1, 2003, 6000);
        db.commit(tx1);
        db.update(tx2, 2003, 7000);

        // readView{trx_list=202,203,204; min_trx_id=202; max_trx_id=205; creator_trx_id=204}
        let version_tx4 = db.select(tx4, 2003, IsolationLevel::RC);
        assert_eq!(version_tx4.unwrap().age, 6000);

        db.commit(tx2);
        db.update(tx3, 2003, 8000);

        // readView{trx_list=203,204; min_trx_id=203; max_trx_id=205; creator_trx_id=204}
        let version_tx4 = db.select(tx4, 2003, IsolationLevel::RC);
        assert_eq!(version_tx4.unwrap().age, 7000);

        db.commit(tx3);

        let version_tx4 = db.select(tx4, 2003, IsolationLevel::RC);
        assert_eq!(version_tx4.unwrap().age, 8000);

        db.commit(tx4);
        db.commit(tx5);
    }

    #[test]
    fn test_repeatable_read() {
        // age = 4000 at the begining.
        let mut db = DB::new(AtomicU32::new(200));
        let tx1 = db.begin();
        let tx2 = db.begin();
        let tx3 = db.begin();
        let tx4 = db.begin();
        let tx5 = db.begin();

        db.update(tx1, 2003, 6000);
        db.commit(tx1);
        db.update(tx2, 2003, 7000);

        let version_tx5 = db.select(tx5, 2003, IsolationLevel::RR);
        assert_eq!(version_tx5.unwrap().age, 6000);

        db.commit(tx2);
        db.update(tx3, 2003, 8000);

        let version_tx5 = db.select(tx5, 2003, IsolationLevel::RR);
        assert_eq!(version_tx5.unwrap().age, 6000);

        db.commit(tx3);
        let version_tx5 = db.select(tx5, 2003, IsolationLevel::RR);
        assert_eq!(version_tx5.unwrap().age, 6000);

        db.commit(tx4);
        db.commit(tx5);
    }
}

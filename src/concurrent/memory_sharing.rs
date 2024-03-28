use std::{
    sync::{Arc, Mutex},
    thread,
};

/**
 * 1. You must attempt to acquire the lock before using the data.
 * 2. When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
 * TODO the dead lock demonstrate.
 */

pub fn new_mutex() -> Mutex<i32> {
    let mutex_num = Mutex::new(2);
    {
        // The call to lock would fail if another thread holding the lock panicked.
        // In that case, no one would ever be able to get the lock,
        // so we’ve chosen to unwrap and have this thread panic if we’re in that situation.
        let b = mutex_num.lock();
        let mut a = b.unwrap();
        // smart pointer
        *a += 1;

        // 3. when the MutexGuard goes out of scope, then will use drop() method to unlock.
    };
    println!("a = {:?}", mutex_num);

    mutex_num
}

pub fn increase_multiple_thread(i: i32) -> i32 {
    let counter = Mutex::new(i);

    // 5. Arc stands for: atomic Rc
    let counter = Arc::new(counter);
    let mut sub_threads = Vec::new();

    for _ in 0..10 {
        // 4. the counter is moved. so we have have to use Arc to help us.
        // 6. use clone , Arc has the same API as Rc
        let counter = Arc::clone(&counter);
        // or
        //let counter = counter.clone();
        let t = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        sub_threads.push(t);
    }

    for thread in sub_threads {
        thread.join().unwrap();
    }

    let res = *counter.lock().unwrap();
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increase_multiple_thread() {
        assert_eq!(10, increase_multiple_thread(0));
    }

    #[test]
    fn test_new_mutex() {
        let res = new_mutex();
        assert_eq!(*res.lock().unwrap(), 3);
    }
}

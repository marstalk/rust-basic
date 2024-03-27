use std::{thread, time::Duration};

pub fn create_thread() {
    thread::spawn(|| {
        for i in 1..100 {
            println!("sub thread run: {}", i);
            thread::sleep(Duration::from_millis(800));
        }
    });

    for i in 1..=100 {
        println!("main thread run: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    // 1. when main thread exit, sub thread will exit too.
}

pub fn join_thread() {
    let t = thread::spawn(|| {
        for i in 1..20 {
            println!("sub thread run: {}", i);
            thread::sleep(Duration::from_millis(800));
        }

        "sub thread exit"
    });

    for i in 1..=20 {
        println!("main thread run: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    // 2. use the join method to wait the thread to be finished and get the return value.
    let t_return = t.join().unwrap();
    println!("sub thread return: {}", t_return);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_thread() {
        join_thread();
    }

    #[test]
    fn test_create_thread() {
        create_thread();
    }
}

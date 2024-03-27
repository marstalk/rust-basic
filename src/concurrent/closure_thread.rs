use std::thread;

pub fn closure_thread() -> i32 {
    let v = vec![1, 2, 3];

    // 1. use the move.
    let t = thread::spawn(move || {
        println!("sub thread run: {:?}", v);
        1
    });

    // v is moved to the closure, so it can't be used after the thread is created.
    t.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_thread() {
        let t = closure_thread();
        assert_eq!(t, 1);
    }
}

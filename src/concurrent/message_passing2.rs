use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub fn iterater_receiver() -> Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("Mars"),
            String::from("Earth"),
            String::from("Mercury"),
        ];
        for msg in msgs {
            sender.send(msg).unwrap();
        }
    });

    receiver
}

pub fn multiple_transmitter() -> Option<Receiver<i32>> {
    let (tx, rx) = mpsc::channel();

    // 2. use clone for multiple producer.
    let tx1 = tx.clone();
    let t = thread::spawn(move || {
        tx1.send(1).unwrap();
    });
    // make sure the 1 goes first
    t.join().unwrap();

    // 3.
    thread::spawn(move || {
        tx.send(2).unwrap();
        tx.send(3).unwrap();
    });

    Some(rx)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiple_transimitter() {
        let res = multiple_transmitter();
        match res {
            Some(receiver) => {
                assert_eq!(receiver.recv(), Ok(1));
                assert_eq!(receiver.recv(), Ok(2));
                assert_eq!(receiver.recv(), Ok(3));
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_iterater_receiver() {
        let receiver = iterater_receiver();
        let mut idx = 0;

        // 1. iterater over the receiver.
        for msg in receiver {
            match idx {
                0 => {
                    assert_eq!(msg, String::from("Mars"))
                }
                1 => {
                    assert_eq!(msg, String::from("Earth"))
                }
                2 => {
                    assert_eq!(msg, String::from("Mercury"))
                }
                _ => assert!(false),
            }
            idx += 1;
        }
    }
}

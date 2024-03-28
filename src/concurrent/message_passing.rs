use std::{
    sync::mpsc::{self, RecvError},
    thread,
};

/**
 * Slogan: Do not communicate by sharing memory; instead, share memory by communicating.
 * channel: 1)transmitter, 2)receiver
 *
 * A channel is said to be closed if either the transmitter or receiver half is dropped.
 *
 * mpsc: multiple producer, single consumer
 *
 */
pub fn mpsc() -> &'static str {
    // 1. mpsc::channel to create transmitter and receiver.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        // 2. sub thread to use transmitter to send message.
        tx.send("Message from sub thread").unwrap();
    });

    // 3. main thread use receiver to get message. [blocking]
    rx.recv().unwrap()
}

pub fn mpsc2() -> (
    Result<i32, RecvError>,
    Result<i32, RecvError>,
    Result<i32, RecvError>,
    Result<i32, RecvError>,
) {
    let (sender, receiver) = mpsc::channel();

    // 4. send multiple message.
    let sub_thread = thread::spawn(move || {
        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();
        drop(sender)
    });

    // 5. wait the sub thread to finished.
    // in fact, because the receiver.recv() is block method, there is no need to sub_thread.join()
    sub_thread.join().unwrap();
    // 6. here, the sender is dropped, but the message is buffered.

    (
        // 7. get the message from channel.
        receiver.recv(),
        receiver.recv(),
        receiver.recv(),
        receiver.recv(),
    )
}

pub fn owner_ship_moved() -> Result<String, RecvError> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Message from sub thread");
        sender.send(msg).unwrap();

        // 8.0 the a is moved. compile failed
        // println!("{}", msg);
    });

    receiver.recv()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_ship_moved() {
        let res = owner_ship_moved();
        if let Ok(mut msg) = res {
            assert_eq!("Message from sub thread", msg);
            msg.push_str(" updated");
            assert_eq!("Message from sub thread updated", msg);
        };
    }

    #[test]
    fn test_mpsc2() {
        let (a, b, c, d) = mpsc2();
        assert_eq!(Ok(1), a);
        assert_eq!(Ok(2), b);
        assert_eq!(Ok(3), c);
        assert_eq!(Err(RecvError), d);
    }

    #[test]
    fn test_mpsc() {
        assert_eq!("Message from sub thread", mpsc());
    }
}

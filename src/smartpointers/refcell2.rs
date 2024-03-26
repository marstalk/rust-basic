/**
 * samples shows that how to use RefCell to do the mock objects.
 */

pub trait Messenger {
    // 1. imagine the Messenger is provided by third party, which the self is immutable.
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage = self.value as f64 / self.max as f64;
        if percentage > 1.0 {
            self.messenger.send("Error, you are over your quota!");
        } else if percentage > 0.8 {
            self.messenger
                .send("Urgent warning, you're over 80% of your quota!");
        } else if percentage > 0.6 {
            self.messenger
                .send("Warning, you're over 60% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // 2. when we implement the trait, we can't change the &self to &mut self.
        fn send(&self, msg: &str) {
            // 3. but we have to change self, so here we turn to RefCell.borrow_mut().
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker {
            messenger: &mock_messenger,
            value: 0,
            max: 100,
        };

        limit_tracker.set_value(70);
        limit_tracker.set_value(81);
        limit_tracker.set_value(101);

        assert_eq!(
            mock_messenger.sent_messages.borrow()[0],
            "Warning, you're over 60% of your quota!"
        );
        assert_eq!(
            mock_messenger.sent_messages.borrow()[1],
            "Urgent warning, you're over 80% of your quota!"
        );
        assert_eq!(
            mock_messenger.sent_messages.borrow()[2],
            "Error, you are over your quota!"
        );
    }
}

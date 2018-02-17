
mod limit;

#[cfg(test)]
mod tests {
    use super::limit::*;
    use std::cell::RefCell;

    #[test]
    fn it_should_have_messenger_trait() {
        #[derive(Debug)]
        struct _DummyMessender {}

        impl Messenger for _DummyMessender {
            fn send(&self, msg: &str) {
                println!("{}", msg);
            }
        }
    }

    #[derive(Debug)]
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_should_have_limit_tracker() {
        let _messenger = MockMessenger::new();

        let _tracker = LimitTracker::new(&_messenger, 100);
    }

    #[test]
    fn it_should_send_warnings_when_hit_75_percent() {
        let messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&messenger, 100);

        tracker.set_value(87);

        assert_eq!(messenger.sent_messages.borrow()[0], "Warning: You\'ve used up over 75% of your quota!");
    }

}

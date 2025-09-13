pub trait Channel<A, B>: std::marker::Copy {
    fn get(&self) -> A;
    fn set(&self, value: B) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let channel = get_test_channel();
        let event: Event = channel.get();
        assert_eq!(event, channel.event);
    }

    #[test]
    fn test_set() {
        let channel = get_test_channel();
        let new_event = Event::Exit;
        let channel = channel.set(new_event);
        assert_eq!(channel.event, new_event);
    }

    fn get_test_channel() -> EventChannel {
        EventChannel {
            event: Event::Keyboard,
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct EventChannel {
        event: Event,
    }

    impl Channel<Event, Event> for EventChannel {
        fn set(&self, value: Event) -> Self {
            let mut copy = *self;
            copy.event = value;
            copy
        }
        fn get(&self) -> Event {
            self.event
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        Keyboard,
        Exit,
    }
}

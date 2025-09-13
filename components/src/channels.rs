use std::marker::Copy;

pub trait Channel<T>: Copy {
    fn get(&self) -> T;
    fn set(&self, value: T) -> Self;
}

pub trait DuelChannel<A, B>: Copy + Channel<A> + Channel<B> {
    fn duel_get(&self) -> A {
        self.get()
    }
    fn duel_set(&self, value: B) -> Self {
        self.set(value)
    }
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

    #[test]
    fn test_duel_get() {
        let channel = get_test_channel();
        let event: Event = channel.duel_get();
        assert_eq!(event, channel.event);
    }

    #[test]
    fn test_duel_set() {
        let channel = get_test_channel();
        let new_event = Event::Exit;
        let channel = channel.duel_set(new_event);
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

    impl Channel<Event> for EventChannel {
        fn set(&self, value: Event) -> Self {
            let mut copy = *self;
            copy.event = value;
            copy
        }
        fn get(&self) -> Event {
            self.event
        }
    }

    impl DuelChannel<Event, Event> for EventChannel {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        Keyboard,
        Exit,
    }
}

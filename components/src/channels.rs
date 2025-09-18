use std::marker::Copy;

pub trait ChannelGet<T>: Copy {
    fn get(&self) -> T;
}

pub trait ChannelSet<T>: Copy {
    fn set(&self, value: T) -> Self;
}

// NOTE: might wanna add ChannelMutSet

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

    const fn get_test_channel() -> EventChannel {
        EventChannel {
            event: Event::Keyboard,
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct EventChannel {
        event: Event,
    }

    impl ChannelGet<Event> for EventChannel {
        fn get(&self) -> Event {
            self.event
        }
    }

    impl ChannelSet<Event> for EventChannel {
        fn set(&self, value: Event) -> Self {
            let mut copy = *self;
            copy.event = value;
            copy
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Event {
        Keyboard,
        Exit,
    }
}

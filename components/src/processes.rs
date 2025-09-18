use std::marker::Copy;
use std::marker::PhantomData;
use crate::channels::ChannelSet;
use crate::channels::ChannelGet;

pub trait Process<C> {
    fn process(&self, channel: C) -> C;
}

pub struct Assign<T> {
    value: T,
}

impl<T> Assign<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T, C> Process<C> for Assign<T> 
where
    T: Copy,
    C: ChannelSet<T>,
{
    fn process(&self, channel: C) -> C {
        channel.set(self.value)
    }
}

pub struct Map<A, B, F> 
where
    F: Fn(A) -> B,
{
    map: F,
    phantom_a: PhantomData<A>,
    phantom_b: PhantomData<B>,
}

impl<A, B, F> Map<A, B, F>  
where
    F: Fn(A) -> B,
{
    pub const fn new(map: F) -> Self {
        Self { map, phantom_a: PhantomData, phantom_b: PhantomData }
    }
}

impl<A, B, F, C> Process<C> for Map<A, B, F> 
where
    A: Copy,
    B: Copy,
    F: Fn(A) -> B,
    C: ChannelGet<A> + ChannelSet<B>,
{
    fn process(&self, channel: C) -> C {
        let a: A = channel.get();
        let b: B = (self.map)(a);
        channel.set(b)
    }
}

pub struct Chain<C> {
    processes: Vec<Box<dyn Process<C>>>,
}

impl<C> Chain<C> {
    pub const fn new(processes: Vec<Box<dyn Process<C>>>) -> Self {
        Self { processes }
    }
}

impl<C> Process<C> for Chain<C> {
    fn process(&self, channel: C) -> C {
        self.processes.iter()
            .fold(channel, |c, p| p.process(c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign() {
        let channel = get_test_channel();
        let new_position = (20, -3);
        let assign = Assign::new(new_position);
        let new_channel = assign.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.position, new_position);

        let channel = new_channel;
        let new_color = Rgb { red: 0, green: 255, blue: 0 };
        let assign = Assign::new(new_color);
        let new_channel = assign.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.color, new_color);
    }

    #[test]
    fn test_map() {
        let channel = get_test_channel();
        let map = Map::new(|color: Rgb| (color.red.into(), color.green.into()));
        let new_channel = map.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!((new_channel.color.red.into(), new_channel.color.green.into()), new_channel.position);
    }

    #[test]
    fn test_chain() {
        let channel = get_test_channel();
        let new_color = Rgb { red: 0, green: 127, blue: 255 };
        let new_position = (32, 0);
        let chain = Chain::new(vec![
            Box::new(Assign::new(new_color)),
            Box::new(Assign::new(new_position)),
        ]);
        let new_channel = chain.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.color, new_color);
        assert_eq!(new_channel.position, new_position);
    }

    const fn get_test_channel() -> GraphicsChannel {
        GraphicsChannel {
            position: (50, 50),
            color: Rgb { red: 255, green: 0, blue: 0 },
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct GraphicsChannel {
        position: (isize, isize),
        color: Rgb,
    }

    impl ChannelGet<Rgb> for GraphicsChannel {
        fn get(&self) -> Rgb {
            self.color
        }
    }

    impl ChannelSet<Rgb> for GraphicsChannel {
        fn set(&self, value: Rgb) -> Self {
            let mut copy = *self;
            copy.color = value;
            copy
        }
    }

    impl ChannelGet<(isize, isize)> for GraphicsChannel {
        fn get(&self) -> (isize, isize) {
            self.position
        }
    }

    impl ChannelSet<(isize, isize)> for GraphicsChannel {
        fn set(&self, value: (isize, isize)) -> Self {
            let mut copy = *self;
            copy.position = value;
            copy
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Rgb {
        red: u8,
        green: u8,
        blue: u8,
    }
}

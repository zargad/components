use crate::channels::Channel;

pub trait Process<C> {
    fn process(&self, channel: C) -> C;
}

pub struct Assign<T> {
    value: T,
}

impl<T> Assign<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T, C> Process<C> for Assign<T> 
where
    T: std::marker::Copy,
    C: Channel<T>,
{
    fn process(&self, channel: C) -> C {
        channel.set(self.value)
    }
}

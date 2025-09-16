use std::marker::Copy;
use std::marker::PhantomData;
use crate::channels::Channel;
use crate::channels::DuelChannel;

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
    T: Copy,
    C: Channel<T>,
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
    pub fn new(map: F) -> Self {
        Self { map, phantom_a: PhantomData, phantom_b: PhantomData }
    }
}

impl<A, B, F, C> Process<C> for Map<A, B, F> 
where
    A: Copy,
    B: Copy,
    F: Fn(A) -> B,
    C: Channel<A> + Channel<B>,
{
    fn process(&self, channel: C) -> C {
        let a: A = channel.get();
        let b: B = (self.map)(a);
        channel.set(b)
    }
}

pub struct Chain<A, B> {
    processes: Vec<Box<dyn DuelChannel<A, B>>>,
}

impl<A, B> Chain<A, B> {
    pub fn new(processes: Vec<Box<dyn DuelChannel<A, B>>>) -> Self {
        Self { processes }
    }
}

impl<A, B, C> Process<C> for Chain<A, B> 
where
    A: Copy,
    B: Copy,
    C: DuelChannel<A, B>,
{
    fn process(&self, channel: C) -> C {
        let mut channel = channel;
        for process in &self.processes {
            channel = process.process(channel);
        }
        channel
    }
}

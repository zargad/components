pub trait Channel<T>: std::marker::Copy {
    fn set(&self, value: T) -> Self;
    fn get(&self) -> T;
}

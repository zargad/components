trait Process<C> {
    fn process(&self, channel: C) -> C;
}

use std::opt::Range;
use crate::components::Component;

trait Screen {
    fn display(&self, range: (Range<isize>, Range<isize>));
}

trait Pixel {
    fn print(&self);
}

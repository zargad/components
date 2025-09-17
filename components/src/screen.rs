use std::ops::Range;
use crate::components::Component;

trait Screen {
    fn display(&self, range: (Range<isize>, Range<isize>));
}

trait Pixel {
    fn print(&self);
}

impl<C> Screen for C
where
    C: Component<(isize, isize), dyn Pixel>,
{
    fn display(&self, range: (Range<isize>, Range<isize>)) {
        let (width, height) = range;
        for y in height.into_iter() {
            for x in width.clone().into_iter() {
                self.get((x, y)).print();
            }
            println!("\x1b[0m")
        }
    }
}

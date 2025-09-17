use std::marker::Copy;
// use std::ops::Range;
use crate::processes::Process;
use crate::channels::Channel;

// NOTE: isize/Point is hardcoded into this module for the time being
type Point = (isize, isize);

pub struct Matrix<const W: usize, const H: usize, T> {
    inner: [[T; W]; H],
}

impl<const W: usize, const H: usize, T> Matrix<W, H, T> {
    pub fn new(inner: [[T; W]; H]) -> Self {
        Self { inner }
    }
    fn get(&self, point: Point) -> Option<&T> {
        let (x, y) = point; 
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;
        Some(self.inner.get(y)?.get(x)?)
    }
}

impl<const W: usize, const H: usize, T, C> Process<C> for Matrix<W, H, T> 
where
    T: Copy + Default,
    C: Channel<T> + Channel<Point>,
{
    fn process(&self, channel: C) -> C {
        let point = channel.get();
        let value = *self.get(point)
            .unwrap_or(&Default::default());
        channel.set(value)
    }
}

// trait Screen {
//     fn display(&self, range: (Range<isize>, Range<isize>));
// }
// 
// trait Pixel {
//     fn print(&self);
// }
// 
// impl<C> Screen for C
// where
//     C: Component<(isize, isize), dyn Pixel>,
// {
//     fn display(&self, range: (Range<isize>, Range<isize>)) {
//         let (width, height) = range;
//         for y in height.into_iter() {
//             for x in width.clone().into_iter() {
//                 self.get((x, y)).print();
//             }
//             println!("\x1b[0m")
//         }
//     }
// }

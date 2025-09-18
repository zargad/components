use std::marker::Copy;
use std::ops::Range;
use std::io::Write;
use std::fmt::Display;
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
        self.inner.get(y)?.get(x)
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

pub trait LinearScreen<T, C>
where
    T: Copy,
    C: Channel<T> + Channel<(isize, isize)> + Default,
{
    fn print(&mut self, value: T);
    fn println(&mut self);
    fn display<P>(&mut self, process: &P, range: (Range<isize>, Range<isize>)) 
    where
        P: Process<C>
    {
        let (width, height) = range;
        for y in height.into_iter() {
            for x in width.clone() {
                let channel: C = Default::default();
                let channel = channel.set((x, y));
                let channel = process.process(channel);
                let pixel = channel.get();
                self.print(pixel);
            }
            self.println();
        }
    }
}

pub struct WriteableScreen<B>
where
    B: Write,
{
    buffer: B,
}

impl<B> WriteableScreen<B>
where
    B: Write,
{
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }
}

impl<T, C, B> LinearScreen<T, C> for WriteableScreen<B>
where
    T: Copy + Display,
    C: Channel<T> + Channel<(isize, isize)> + Default,
    B: Write,
{
    fn print(&mut self, value: T) {
        write!(self.buffer, "{}", value).unwrap(); // TODO: Figure out whether the error should
                                                     // be propagated
    }
    fn println(&mut self) {
        writeln!(self.buffer, "\x1b[0m").unwrap(); // TODO: Figure out whether the error should
                                                   // be propagated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_matrix_get() {
        let matrix = get_test_matrix();
        let value = matrix.get((2, 0));
        assert_eq!(value, Some(&A));
        let value = matrix.get((0, 2));
        assert_eq!(value, Some(&B));
        let value = matrix.get((3, -3));
        assert_eq!(value, None);
        let value = matrix.get((0, 1000));
        assert_eq!(value, None);
    }

    #[test]
    fn test_matrix_process() {
        let matrix = get_test_matrix();
        let channel = SimpleChannel::new((2, 0));
        let new_channel = matrix.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.value, A);
        let channel = SimpleChannel::new((0, 2));
        let new_channel = matrix.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.value, B);
        let channel = SimpleChannel::new((3, -3));
        let new_channel = matrix.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.value, 0);
        let channel = SimpleChannel::new((0, 1000));
        let new_channel = matrix.process(channel);
        assert_ne!(new_channel, channel, "the test is invalid because the value of `channel` did not change after being processed");
        assert_eq!(new_channel.value, 0);
    }

    #[test]
    fn test_writeable_screen() {
        let matrix = get_test_matrix();
        let mut writeable_screen = WriteableScreen::new(Cursor::new(Vec::new()));
        <WriteableScreen<Cursor<Vec<u8>>> as LinearScreen<i32, SimpleChannel>>::display::<Matrix<4, 4, i32>>(&mut writeable_screen, &matrix, (0..3, -1..3));
        let result = String::from_utf8(writeable_screen.buffer.into_inner()).unwrap();
        let expected = format!("000{C}33{A}{C}333{C}{B}33{C}", C="\x1b[0m\n");
        assert_eq!(result, expected);
    }

    const A: i32 = 1;
    const B: i32 = 2;

    fn get_test_matrix() -> Matrix<4, 4, i32> {
        Matrix::new([
            [3, 3, A, 3],
            [3, 3, 3, 3],
            [B, 3, 3, 3],
            [3, 3, 3, 3],
        ])
    }

    #[derive(Debug, Clone, Copy, PartialEq, Default)]
    struct SimpleChannel {
        position: Point,
        value: i32,
    }

    impl SimpleChannel {
        fn new(position: Point) -> Self {
            Self { position, value: 100 }
        }
    }

    impl Channel<Point> for SimpleChannel {
        fn get(&self) -> Point {
            self.position
        }
        fn set(&self, value: Point) -> Self {
            let mut copy = *self;
            copy.position = value;
            copy
        }
    }

    impl Channel<i32> for SimpleChannel {
        fn get(&self) -> i32 {
            self.value
        }
        fn set(&self, value: i32) -> Self {
            let mut copy = *self;
            copy.value = value;
            copy
        }
    }
}

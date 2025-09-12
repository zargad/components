pub trait Component<I, T> {
    fn get(&self, input: I) -> T;
}

pub struct Uniform<T> {
    value: T,
}

impl<T> Uniform<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<I, T> Component<I, T> for Uniform<T>
where 
    T: std::marker::Copy,
{
    fn get(&self, _input: I) -> T {
        self.value
    }
}

pub struct Matrix<const W: usize, const H: usize, T> {
    base: [[T; W]; H],
}

impl<const W: usize, const H: usize, T> Matrix<W, H, T> {
    pub fn new(base: [[T; W]; H]) -> Self {
        Self { base }
    }
}

impl<const W: usize, const H: usize, T> Component<(isize, isize), T> for Matrix<W, H, T> 
where 
    T: std::marker::Copy + Default,
{
    fn get(&self, input: (isize, isize)) -> T {
        let (x, y) = input;
        let Ok(x) = usize::try_from(x) else { return Default::default() };
        let Ok(y) = usize::try_from(y) else { return Default::default() };
        if x >= W || y >= H {
            return Default::default();
        }
        self.base[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform() {
        let uniform = Uniform::new(100);
        let value = uniform.get("mystring");
        assert_eq!(value, 100);
    }

    #[test]
    fn test_matrix() {
        let x = false;
        let z = true;
        let matrix = Matrix::new([
            [z, x, x, z],
            [x, x, x, x],
            [z, x, x, z],
            [x, z, z, x],
        ]);
        let value = matrix.get((3, 0));
        assert_eq!(value, true);
        let value = matrix.get((2, 2));
        assert_eq!(value, false);
        let value = matrix.get((0, 2));
        assert_eq!(value, true);
        let value = matrix.get((4, 3));
        assert_eq!(value, Default::default());
        let value = matrix.get((100, 0));
        assert_eq!(value, Default::default());
        let value = matrix.get((-1000, -1000));
        assert_eq!(value, Default::default());
    }
}

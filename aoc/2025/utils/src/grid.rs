use core::fmt;
use std::{
    collections::HashSet,
    fmt::Display,
    mem,
    ops::{Index, IndexMut, Neg},
};

use crate::types::Both;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecMat<T> {
    data: Vec<Vec<T>>,
}

impl<T> IndexMut<Point<usize>> for VecMat<T> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        self.get_mut(index)
            .expect("indexed point must be in bounds")
    }
}

impl<T> Index<Point<usize>> for VecMat<T> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        self.get(index).expect("indexed point must be in bounds")
    }
}
impl<T> Index<usize> for VecMat<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.get_row(index).expect("indexed row must be in bounds")
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for VecMat<T> {
    type Error = Vec<Vec<T>>;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

pub struct MatDisplayWith<'a, T, F> {
    grid: &'a VecMat<T>,
    f: F,
}

impl<'a, T, F> MatDisplayWith<'a, T, F> {
    pub fn new(grid: &'a VecMat<T>, f: F) -> Self {
        Self { grid, f }
    }

    pub fn fmt_highlight(
        &self,
        f: &mut fmt::Formatter<'_>,
        highlights: &HashSet<Point<usize>>,
    ) -> fmt::Result
    where
        T: fmt::Display,
    {
        for (i, row) in self.grid.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if highlights.contains(&Point::new(j, i)) {
                    write!(f, "\x1b[1;31m{c}\x1b[0m")?;
                } else {
                    write!(f, "{c}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, F, U> Display for MatDisplayWith<'_, T, F>
where
    F: Fn(usize, usize, &T) -> U,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.grid.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                write!(f, "{}", (self.f)(j, i, c))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for VecMat<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.formatter_with(|_, _, c| *c).fmt(f)
    }
}

impl Display for VecMat<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn aschar(_: usize, _: usize, c: &u8) -> char {
            *c as char
        }
        self.formatter_with(aschar).fmt(f)
    }
}

impl<T> VecMat<T> {
    pub fn formatter(&self) -> MatDisplayWith<'_, T, fn(usize, usize, &T) -> &T> {
        MatDisplayWith::new(self, |_, _, c| c)
    }

    pub fn formatter_with<'a, F, U>(&'a self, f: F) -> MatDisplayWith<'a, T, F>
    where
        F: Fn(usize, usize, &T) -> U,
    {
        MatDisplayWith::new(self, f)
    }

    pub fn filled(shape: (usize, usize), val: &T) -> Self
    where
        T: Clone,
    {
        let (rows, cols) = shape;
        let data = vec![vec![val.clone(); cols]; rows];
        Self { data }
    }

    pub fn filled_with(shape: (usize, usize), f: impl Fn(Point<usize>) -> T) -> Self {
        let (rows, cols) = shape;
        let data = (0..rows)
            .map(|y| (0..cols).map(|x| f(Point::new(x, y))).collect())
            .collect();
        Self { data }
    }

    pub fn highlighted<'a, 'b: 'a>(
        &'a self,
        highlights: &'b HashSet<Point<usize>>,
    ) -> impl fmt::Display + 'a
    where
        T: fmt::Display,
    {
        struct Highlighted<'a, T> {
            grid: &'a VecMat<T>,
            highlights: &'a HashSet<Point<usize>>,
        }

        impl<T> fmt::Display for Highlighted<'_, T>
        where
            T: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.grid.formatter().fmt_highlight(f, self.highlights)
            }
        }

        Highlighted {
            grid: self,
            highlights,
        }
    }

    pub fn new(data: Vec<Vec<T>>) -> Result<Self, Vec<Vec<T>>> {
        let rows = data.len();
        if rows != 0 {
            let cols = data[0].len();
            if cols == 0 {
                // invalid empty matrix
                return Err(data);
            }
            if data.iter().any(|row| row.len() != cols) {
                return Err(data);
            }
        }
        Ok(Self { data })
    }

    // construct a matrix from an iterator (rows) of iterable (columns)
    pub fn from_nestiter<I: IntoIterator<Item = T>>(
        iter: impl Iterator<Item = I>,
    ) -> Result<Self, Vec<Vec<T>>> {
        let data = iter.map(|x| x.into_iter().collect()).collect();
        Self::new(data)
    }

    pub fn for_each(&self, mut f: impl FnMut(Point<usize>, &T)) {
        for (y, row) in self.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                f(Point::new(x, y), cell);
            }
        }
    }

    pub fn for_each_mut(&mut self, mut f: impl FnMut(Point<usize>, &mut T)) {
        for (y, row) in self.data.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                f(Point::new(x, y), cell);
            }
        }
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|row| row.as_slice())
    }

    pub fn cols(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn get_row(&self, idx: usize) -> Option<&[T]> {
        self.data.get(idx).map(Vec::as_slice)
    }
    pub fn get_row_mut(&mut self, idx: usize) -> Option<&mut [T]> {
        // it's fine to give a &mut [T] reference since
        // that means callers can't change its length
        self.data.get_mut(idx).map(Vec::as_mut_slice)
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    pub fn iter_pos(&self) -> impl Iterator<Item = (Point<usize>, &T)> {
        (0..self.rows()).flat_map(move |y| {
            (0..self.cols()).map(move |x| {
                let p = Point::new(x, y);
                (p, self.get(p).unwrap())
            })
        })
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set(&mut self, p: Point<usize>, val: T) -> T {
        mem::replace(self.get_mut(p).expect("should be in bounds"), val)
    }

    pub fn get(&self, p: Point<usize>) -> Option<&T> {
        self.data.get(p.y).and_then(|row| row.get(p.x))
    }

    pub fn get_mut(&mut self, p: Point<usize>) -> Option<&mut T> {
        self.data.get_mut(p.y).and_then(|row| row.get_mut(p.x))
    }

    pub fn find(&self, value: &T) -> Option<Point<usize>>
    where
        T: PartialEq,
    {
        for (y, row) in self.data.iter().enumerate() {
            if let Some(x) = row.iter().position(|v| v == value) {
                return Some(Point::new(x, y));
            }
        }
        None
    }

    pub fn iter_all(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|v| v.iter())
    }

    /// Obtain a new matrix whose rows are the coulmns of the current matrix
    /// and vice versa
    pub fn transpose(&self) -> Self
    where
        T: Clone,
    {
        let data = (0..self.cols())
            .map(|c| (0..self.rows()).map(|r| self.data[r][c].clone()).collect())
            .collect();
        let mat = Self { data };
        // we don't need to check dimensions since it's aleady a matrix
        debug_assert!(mat.data.len() == 0 || mat.data.iter().all(|x| x.len() == mat.data[0].len()));
        mat
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Matrix<T> {
    pub fn new(data: &[T], width: usize) -> (Self, &[T])
    where
        T: Clone,
    {
        let end = data.len() - data.len() % width;
        let (used, rest) = data.split_at(end);
        let mat = Self {
            data: used.to_vec(),
            width,
        };

        (mat, rest)
    }

    pub fn get(&self, p: Point<usize>) -> Option<&T> {
        self.data.get(self.checked_idx_for(p.y, p.x)?)
    }

    pub fn get_mut(&mut self, p: Point<usize>) -> Option<&mut T> {
        let idx = self.checked_idx_for(p.y, p.x)?;
        self.data.get_mut(idx)
    }

    pub fn checked_idx_for(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height() || col >= self.width() {
            return None;
        }
        Some(self.idx_for(row, col))
    }

    pub fn idx_for(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Delta<T> {
    pub dx: T,
    pub dy: T,
}

impl<T: Neg> Neg for Delta<T> {
    type Output = Delta<T::Output>;

    fn neg(self) -> Self::Output {
        Delta::new(-self.dx, -self.dy)
    }
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Delta<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }

    pub fn scale<U>(&self, factor: T) -> Delta<U>
    where
        T: Copy + std::ops::Mul<Output = U>,
    {
        Delta::new(self.dx * factor, self.dy * factor)
    }
}

impl Delta<isize> {
    pub fn manhattan(&self) -> usize {
        self.dx.abs() as usize + self.dy.abs() as usize
    }

    pub fn dir(&self) -> Option<Dir> {
        Some(if self.dx > 0 {
            if self.dy > 0 {
                Dir::NE
            } else if self.dy < 0 {
                Dir::SE
            } else {
                Dir::E
            }
        } else if self.dx < 0 {
            if self.dy > 0 {
                Dir::NW
            } else if self.dy < 0 {
                Dir::SW
            } else {
                Dir::W
            }
        } else if self.dy > 0 {
            Dir::N
        } else if self.dy < 0 {
            Dir::S
        } else {
            None?
        })
    }
}

impl<T> Point<T> {
    pub fn into_both(self) -> Both<T, T> {
        Both(self.x, self.y)
    }
}

impl Point<isize> {
    pub fn add(&self, delta: Delta<isize>) -> Self {
        Self {
            x: self.x + delta.dx,
            y: self.y + delta.dy,
        }
    }
}
impl Point<u8> {
    pub fn delta_to(&self, other: Self) -> Option<Delta<i8>> {
        Some(Delta::new(
            (other.x as i8).checked_sub(self.x as i8)?,
            (other.y as i8).checked_sub(self.y as i8)?,
        ))
    }
}

impl Point<usize> {
    pub fn delta_to(&self, other: Self) -> Option<Delta<isize>> {
        Some(Delta::new(
            (other.x as isize).checked_sub(self.x as isize)?,
            (other.y as isize).checked_sub(self.y as isize)?,
        ))
    }
    pub fn wrapping_add_signed(&self, d: Delta<isize>) -> Self {
        Self {
            x: self.x.wrapping_add_signed(d.dx),
            y: self.y.wrapping_add_signed(d.dy),
        }
    }

    pub fn checked_add_signed(&self, d: Delta<isize>) -> Option<Self> {
        let x = self.x.checked_add_signed(d.dx)?;
        let y = self.y.checked_add_signed(d.dy)?;
        Some(Self { x, y })
    }

    /// Check if the point is in bound for a chart with given size.
    pub fn in_bounds(&self, (width, height): (usize, usize)) -> bool {
        self.x < width && self.y < height
    }

    /// Some(self) if the point is in bounds, else None
    pub fn as_in_bounds(&self, (width, height): (usize, usize)) -> Option<Self> {
        self.in_bounds((width, height)).then_some(*self)
    }

    /// The point's neighbours in the 4 cardinal directions
    /// (wrapping to Self's type width on overflow)
    pub fn cross_neighbours(&self) -> [Self; 4] {
        let p = *self;
        Dir::CROSS.map(move |dir| p.neighbour(dir))
    }

    /// The point's neighbours in all 8 directions
    /// (wrapping to Self's type width on overflow)
    pub fn all_neighbours(&self) -> [Self; 8] {
        let p = *self;
        Dir::ALL.map(move |dir| p.neighbour(dir))
    }
    /// Get the point neighbour in the given Dir
    /// Wraps to Self's type width on overflow
    pub fn neighbour(&self, dir: Dir) -> Self {
        self.wrapping_add_signed(dir.to_delta())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub const ALL: [Dir; 8] = [
        Dir::N,
        Dir::NE,
        Dir::E,
        Dir::SE,
        Dir::S,
        Dir::SW,
        Dir::W,
        Dir::NW,
    ];

    pub const CROSS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];
    pub const HORIZONTAL: [Dir; 2] = [Dir::E, Dir::W];
    pub const VERTICAL: [Dir; 2] = [Dir::N, Dir::S];

    pub fn id(&self) -> u8 {
        match self {
            Dir::N => 0,
            Dir::NE => 1,
            Dir::E => 2,
            Dir::SE => 3,
            Dir::S => 4,
            Dir::SW => 5,
            Dir::W => 6,
            Dir::NW => 7,
        }
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Dir::N | Dir::S)
    }
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Dir::E | Dir::W)
    }

    pub fn from_id(id: u8) -> Option<Self> {
        Self::ALL.get(id as usize).copied()
    }

    pub fn to_offset<T: From<i8>>(&self) -> (T, T) {
        let (a, b) = self.to_i8_offset();
        (a.into(), b.into())
    }

    pub fn to_i8_offset(&self) -> (i8, i8) {
        match self {
            Dir::N => (0, -1),
            Dir::NE => (1, -1),
            Dir::E => (1, 0),
            Dir::SE => (1, 1),
            Dir::S => (0, 1),
            Dir::SW => (-1, 1),
            Dir::W => (-1, 0),
            Dir::NW => (-1, -1),
        }
    }

    pub fn rotate_by(&self, n: isize) -> Self {
        let id = (self.id() as isize + n).rem_euclid(8) as u8;
        Self::from_id(id).unwrap()
    }

    pub fn clockwise(&self) -> Self {
        self.rotate_by(1)
    }

    pub fn counterclockwise(&self) -> Self {
        self.rotate_by(-1)
    }

    pub fn clockwise_cross(&self) -> Self {
        self.rotate_by(2)
    }

    pub fn counterclockwise_cross(&self) -> Self {
        self.rotate_by(-2)
    }

    pub fn to_delta<T: From<i8>>(&self) -> Delta<T> {
        let (dx, dy) = self.to_offset();
        Delta::new(dx, dy)
    }

    pub fn opposite(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::NE => Dir::SW,
            Dir::E => Dir::W,
            Dir::SE => Dir::NW,
            Dir::S => Dir::N,
            Dir::SW => Dir::NE,
            Dir::W => Dir::E,
            Dir::NW => Dir::SE,
        }
    }
}

impl<T> Into<Both<T, T>> for Point<T> {
    fn into(self) -> Both<T, T> {
        self.into_both()
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::N => '↑',
            Dir::NE => '↗',
            Dir::E => '→',
            Dir::SE => '↘',
            Dir::S => '↓',
            Dir::SW => '↙',
            Dir::W => '←',
            Dir::NW => '↖',
        };
        write!(f, "{}", c)
    }
}

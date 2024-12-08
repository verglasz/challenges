use core::fmt;
use std::{collections::HashSet, ops::Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecMat<T> {
    data: Vec<Vec<T>>,
}

impl<T> TryFrom<Vec<Vec<T>>> for VecMat<T> {
    type Error = Vec<Vec<T>>;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for VecMat<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for &c in row {
                write!(f, "{}", c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> VecMat<T> {
    pub fn fmt_highlight(
        &self,
        f: &mut fmt::Formatter<'_>,
        highlights: &HashSet<Point<usize>>,
    ) -> fmt::Result
    where
        T: fmt::Display,
    {
        for (i, row) in self.data.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                let p = Point::new(j, i);
                if highlights.contains(&p) {
                    write!(f, "\x1b[1;31m{c}\x1b[0m")?;
                } else {
                    write!(f, "{c}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
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
                self.grid.fmt_highlight(f, self.highlights)
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
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Delta<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }
}

impl Point<usize> {
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

    pub fn in_bounds(&self, (width, height): (usize, usize)) -> bool {
        self.x < width && self.y < height
    }

    pub fn as_in_bounds(&self, (width, height): (usize, usize)) -> Option<Self> {
        self.in_bounds((width, height)).then_some(*self)
    }

    pub fn delta_to(&self, other: Self) -> Option<Delta<isize>> {
        Some(Delta::new(
            (other.x as isize).checked_sub(self.x as isize)?,
            (other.y as isize).checked_sub(self.y as isize)?,
        ))
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

    pub fn to_offset(&self) -> (isize, isize) {
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

    pub fn clockwise(&self) -> Self {
        match self {
            Dir::N => Dir::NE,
            Dir::NE => Dir::E,
            Dir::E => Dir::SE,
            Dir::SE => Dir::S,
            Dir::S => Dir::SW,
            Dir::SW => Dir::W,
            Dir::W => Dir::NW,
            Dir::NW => Dir::N,
        }
    }

    pub fn counterclockwise(&self) -> Self {
        match self {
            Dir::N => Dir::NW,
            Dir::NE => Dir::N,
            Dir::E => Dir::NE,
            Dir::SE => Dir::E,
            Dir::S => Dir::SE,
            Dir::SW => Dir::S,
            Dir::W => Dir::SW,
            Dir::NW => Dir::W,
        }
    }

    pub fn clockwise_cross(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::NE => Dir::SE,
            Dir::SE => Dir::SW,
            Dir::SW => Dir::NW,
            Dir::NW => Dir::NE,
        }
    }

    pub fn counterclockwise_cross(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::NE => Dir::NW,
            Dir::NW => Dir::SW,
            Dir::SW => Dir::SE,
            Dir::SE => Dir::NE,
        }
    }

    pub fn to_delta(&self) -> Delta<isize> {
        let (dx, dy) = self.to_offset();
        Delta::new(dx, dy)
    }
}

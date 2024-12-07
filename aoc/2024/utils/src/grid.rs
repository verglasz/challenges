#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VecMat<T> {
    data: Vec<Vec<T>>,
}

impl<T> VecMat<T> {
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

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get(&self, p: Point<usize>) -> Option<&T> {
        self.data.get(p.y).and_then(|row| row.get(p.x))
    }

    pub fn get_mut(&mut self, p: Point<usize>) -> Option<&mut T> {
        self.data.get_mut(p.y).and_then(|row| row.get_mut(p.x))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Delta<T> {
    pub dx: T,
    pub dy: T,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn to_delta(&self) -> Delta<isize> {
        let (dx, dy) = self.to_offset();
        Delta::new(dx, dy)
    }
}

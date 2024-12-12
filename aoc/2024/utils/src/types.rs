/// Most generic sum type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn left(&self) -> Option<&L> {
        use Either::*;
        match self {
            Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&R> {
        use Either::*;
        match self {
            Right(r) => Some(r),
            _ => None,
        }
    }

    pub fn left_mut(&mut self) -> Option<&mut L> {
        use Either::*;
        match self {
            Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn right_mut(&mut self) -> Option<&mut R> {
        use Either::*;
        match self {
            Right(r) => Some(r),
            _ => None,
        }
    }

    pub fn into_left(self) -> Option<L> {
        use Either::*;
        match self {
            Left(l) => Some(l),
            _ => None,
        }
    }

    pub fn into_right(self) -> Option<R> {
        use Either::*;
        match self {
            Right(r) => Some(r),
            _ => None,
        }
    }

    pub fn swap(self) -> Either<R, L> {
        use Either::*;
        match self {
            Left(l) => Right(l),
            Right(r) => Left(r),
        }
    }

    pub fn map_left<F, T>(self, f: F) -> Either<T, R>
    where
        F: FnOnce(L) -> T,
    {
        use Either::*;
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(r),
        }
    }

    pub fn map_right<F, T>(self, f: F) -> Either<L, T>
    where
        F: FnOnce(R) -> T,
    {
        use Either::*;
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }

    pub fn map_either<F, G, T, V>(self, f: F, g: G) -> Either<T, V>
    where
        F: FnOnce(L) -> T,
        G: FnOnce(R) -> V,
    {
        use Either::*;
        match self {
            Left(l) => Left(f(l)),
            Right(r) => Right(g(r)),
        }
    }

    pub fn as_ref(&self) -> Either<&L, &R> {
        use Either::*;
        match self {
            Left(l) => Left(l),
            Right(r) => Right(r),
        }
    }

    pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        use Either::*;
        match self {
            Left(l) => Left(l),
            Right(r) => Right(r),
        }
    }
}

// Most generic product type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Both<L, R>(pub L, pub R);

impl<Ls, Rs, L, R> FromIterator<Either<L, R>> for Both<Ls, Rs>
where
    Ls: From<Vec<L>>,
    Rs: From<Vec<R>>,
{
    fn from_iter<T: IntoIterator<Item = Either<L, R>>>(iter: T) -> Self {
        let (ls, rs) = iter
            .into_iter()
            .fold((vec![], vec![]), |(mut ls, mut rs), e| {
                match e {
                    Either::Left(l) => ls.push(l),
                    Either::Right(r) => rs.push(r),
                }
                (ls, rs)
            });
        Both(ls.into(), rs.into())
    }
}

impl<L, R> Both<L, R> {
    pub fn left(&self) -> &L {
        &self.0
    }

    pub fn right(&self) -> &R {
        &self.1
    }

    pub fn left_mut(&mut self) -> &mut L {
        &mut self.0
    }

    pub fn right_mut(&mut self) -> &mut R {
        &mut self.1
    }

    pub fn swap(self) -> Both<R, L> {
        Both(self.1, self.0)
    }

    pub fn map_left<T>(self, f: impl FnOnce(L) -> T) -> Both<T, R> {
        Both(f(self.0), self.1)
    }

    pub fn map_right<T>(self, f: impl FnOnce(R) -> T) -> Both<L, T> {
        Both(self.0, f(self.1))
    }

    pub fn map_both<T, U>(self, f: impl FnOnce(L) -> T, g: impl FnOnce(R) -> U) -> Both<T, U> {
        Both(f(self.0), g(self.1))
    }
}

pub trait SumEither<L, R> {
    fn either(self) -> Either<L, R>;
}
impl<T, E> SumEither<T, E> for Result<T, E> {
    fn either(self) -> Either<T, E> {
        use Either::*;
        match self {
            Ok(l) => Left(l),
            Err(r) => Right(r),
        }
    }
}

pub mod ext {
    use super::*;

    pub trait SideEitherExt<L, R> {
        fn left(self) -> Either<L, ()>;
        fn right(self) -> Either<(), R>;
    }

    impl<T> SideEitherExt<T, T> for Option<T> {
        fn left(self) -> Either<T, ()> {
            use Either::*;
            match self {
                Some(l) => Left(l),
                _ => Right(()),
            }
        }

        fn right(self) -> Either<(), T> {
            use Either::*;
            match self {
                Some(r) => Right(r),
                _ => Left(()),
            }
        }
    }
}

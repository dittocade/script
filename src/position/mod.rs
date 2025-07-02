use std::{num::TryFromIntError, ops::{Add, Mul, Sub}};

#[derive(Default, Debug, Clone, Copy)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn into_tuple(self) -> (T, T, T) {
        (self.x, self.y, self.z)
    }

    pub fn into_slice(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T: Copy> Position<T> {
    pub fn to_reversed(&self) -> Self {
        Self::new(self.z, self.y, self.x)
    }
}

impl<T> Into<(T, T, T)> for Position<T> {
    fn into(self) -> (T, T, T) {
        self.into_tuple()
    }
}

impl<T> Into<[T; 3]> for Position<T> {
    fn into(self) -> [T; 3] {
        self.into_slice()
    }
}

impl TryInto<Position<i32>> for Position<usize> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Position<i32>, Self::Error> {
        Ok(Position {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
            z: self.z.try_into()?,
        })
    }
}

impl TryInto<Position<usize>> for Position<i32> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Position<usize>, Self::Error> {
        Ok(Position {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
            z: self.z.try_into()?,
        })
    }
}

impl TryInto<Position<u16>> for Position<i32> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Position<u16>, Self::Error> {
        Ok(Position {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
            z: self.z.try_into()?,
        })
    }
}

impl TryInto<Position<usize>> for Position<u16> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Position<usize>, Self::Error> {
        Ok(Position {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
            z: self.z.try_into()?,
        })
    }
}

impl<T> From<(T, T, T)> for Position<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2
        }
    }
}

impl<T: Copy> From<[T; 3]> for Position<T> {
    fn from(value: [T; 3]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2]
        }
    }
}

impl<T: Add<Output = T>> Add for Position<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Position<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Position<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
use std::{fmt::Debug, ops::{Index, IndexMut}};

pub trait Matrix<Idx, T> {
    fn new(dimensions: Idx, data: Vec<T>) -> Self where Self: Sized;
    fn with_default(dimensions: Idx) -> Self where Self: Sized, T: Default + Clone {
        let capacity = Self::calculate_capacity(&dimensions);
        Self::new(
            dimensions,
            vec![T::default(); capacity]
        )
    }
    fn calculate_capacity(dimensions: &Idx) -> usize where Self: Sized;
    fn get_capacity(&self) -> usize where Self: Sized {
        Self::calculate_capacity(&self.get_dimensions())
    }
    fn get_index(&self, position: Idx) -> usize;
    fn get_data(&self) -> &Vec<T>;
    fn get_dimensions(&self) -> Idx;
    fn get_mut_data(&mut self) -> &mut Vec<T>;
}

impl<Idx, T > Index<Idx> for dyn Matrix<Idx, T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.get_data()[self.get_index(index)]
    }
}

impl<Idx, T > IndexMut<Idx> for dyn Matrix<Idx, T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let index = self.get_index(index);
        &mut self.get_mut_data()[index]
    }
}

#[derive(Debug)]
pub struct Matrix3<T> {
    pub data: Vec<T>,
    pub dimensions: [usize; 3],
}

impl<T> Matrix<[usize; 3], T> for Matrix3<T> {
    fn new(dimensions: [usize; 3], data: Vec<T>) -> Self {
        Self { 
            dimensions,
            data,
        }
    }

    fn get_mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_dimensions(&self) -> [usize; 3] {
        self.dimensions
    }

    fn calculate_capacity([x, y, z]: &[usize; 3]) -> usize {
        x * y * z
    }

    fn get_index(self: &Self, [x, y, z]: [usize; 3]) -> usize {
        let index = x;
        let index = index * self.dimensions[1] + y;
        let index = index * self.dimensions[2] + z;
        index
    }
}

#[derive(Debug)]
pub struct Matrix4<T> {
    pub data: Vec<T>,
    pub dimensions: [usize; 4],
}

impl<T> Matrix<[usize; 4], T> for Matrix4<T> {
    fn new(dimensions: [usize; 4], data: Vec<T>) -> Self {
        Self { 
            dimensions,
            data,
        }
    }

    fn get_mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    fn get_dimensions(&self) -> [usize; 4] {
        self.dimensions
    }

    fn calculate_capacity([x, y, z, a]: &[usize; 4]) -> usize {
        x * y * z * a
    }

    fn get_capacity(&self) -> usize where Self: Sized {
        Self::calculate_capacity(&self.dimensions)
    }


    fn get_index(self: &Self, [x, y, z, a]: [usize; 4]) -> usize {
        let index = x;
        let index = index * self.dimensions[1] + y;
        let index = index * self.dimensions[2] + z;
        let index = index * self.dimensions[3] + a;
        index
    }
}
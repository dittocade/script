
#[derive(Debug)]
pub struct Matrix3<T> {
    pub data: Vec<T>,
    pub dimensions: [usize; 3],
}

impl<T> Matrix3<T> {
    pub fn new(dimensions: [usize; 3], data: Vec<T>) -> Self {
        Self {
            data,
            dimensions,
        }
    }

    pub fn with_dimensions(dimensions @ [x, y, z]: [usize; 3]) -> Self {
        Self {
            data: Vec::with_capacity(x * y * z),
            dimensions,
        }
    }

    fn index(self: &Self, position @ [x, y, z]: [usize; 3]) -> usize {
        let index = x;
        let index = index * self.dimensions[1] + y;
        let index = index * self.dimensions[2] + z;
        index
    }

    pub fn get(self: &Self, position: [usize; 3]) -> &T {
        &self.data[self.index(position)]
    }

    pub fn set(self: &mut Self, position: [usize; 3], data: T) {
        let index = self.index(position);
        self.data[index] = data;
    }
}
#[derive(Debug)]
pub struct Vector<const D: usize> {
    components: [f32; D],
}

#[derive(Debug)]
pub struct Region<const D: usize> {
    pub size: Vector<D>,
}

impl<const D: usize> Vector<D> {
    pub fn new() -> Vector<D> {
        Vector { components: [0_f32; D] }
    }

    pub fn components<'a>(&'a self) -> &'a [f32; D] {
        &self.components
    }

    pub fn component_sum(&self) -> f32 {
        self.components()
            .iter()
            .sum()
    }

    pub fn from(components: [f32; D]) -> Vector<D> {
        Vector { components }
    }

    pub fn plus(&mut self, other: &Vector<D>) {
        let other_components = other.components();
        for i in 0..D {
            self.components[i] += other_components[i];
        }
    }

    pub fn new_scaled_by(&self, factor: f32) -> Vector<D> {
        let mut result = Vector::from(self.components().clone());
        result.multiply_by(factor);
        result
    }

    pub fn multiply_by(&mut self, factor: f32) {
        for i in 0..D {
            self.components[i] *= factor;
        }
    }

    pub fn dot(&self, other: &Vector<D>) -> f32 {
        let i_self = self.components().iter();
        let i_other = other.components().iter();
        i_self.zip(i_other)
            .map(|(a, b)| a * b)
            .sum()
    }

    pub fn difference(&self, other: &Vector<D>) -> Vector<D> {
        let mut c = [0_f32; D];
        for i in 0..D {
            c[i] = self.components()[i] - other.components()[i];
        }
        Vector::from(c)
    }

    pub fn vector_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn squared_length(&self) -> f32 {
        self.vector_squared()
    }
}

impl<const D: usize> Region<D> {
    pub fn new(size: [f32; D]) -> Region<D> {
        Region { size: Vector::from(size) }
    }

    pub fn wrap(&self, coords: &mut Vector<D>) {
        let mut shift = [0_f32; D];
        for i in 0..D {
            let component = coords.components()[i];
            let size_i = self.size.components()[i];
            if component >= size_i / 2_f32 {
                shift[i] = -size_i;
            } else if component < -size_i / 2_f32 {
                shift[i] = size_i;
            }
        }
        coords.plus(&Vector::from(shift));
    }

    pub fn get_shift(&self, factor: f32) -> Vector<D> {
        self.size.new_scaled_by(factor)
    }
}
#[derive(Debug)]
pub struct Vector<const D: usize> {
    components: [f32; D],
}

#[derive(Debug)]
pub struct Region<const D: usize> {
    size: Vector<D>,
}

impl<const D: usize> Vector<D> {
    pub fn new() -> Vector<D> {
        Vector { components: [0_f32; D] }
    }

    pub fn components<'a>(&'a self) -> &'a [f32; D] {
        &self.components
    }

    pub fn from(components: [f32; D]) -> Vector<D> {
        Vector { components }
    }

    pub fn from_scaled(vect: &Vector<D>, factor: f32) -> Vector<D> {
        let mut result = Vector::from(vect.components().clone());
        result.multiply_by(factor);
        result
    }

    pub fn plus(&mut self, other: &Vector<D>) {
        let other_components = other.components();
        for i in 0..D {
            self.components[i] += other_components[i];
        }
    }

    pub fn multiply_by(&mut self, c: f32) {
        for i in 0..D {
            self.components[i] *= c;
        }
    }

    pub fn dot(&self, other: &Vector<D>) -> f32 {
        let i_self = self.components().iter();
        let i_other = other.components().iter();
        i_self.zip(i_other)
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl<const D: usize> Region<D> {
    pub fn new(size: [f32; D]) -> Region<D> {
        Region { size: Vector::from(size) }
    }

    pub fn wrap(&self, r: &mut Vector<D>) {
        let mut shift = [0_f32; D];
        for i in 0..D {
            let component = r.components()[i];
            let size_i = self.size.components()[i];
            if component >= size_i / 2_f32 {
                shift[i] = -size_i;
            } else if component < -size_i / 2_f32 {
                shift[i] = size_i;
            }
        }
        r.plus(&Vector::from(shift));
    }
}
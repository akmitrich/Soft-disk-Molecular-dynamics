#[derive(Debug)]
pub struct Vector<const D: usize> {
    components: [f32; D],
}

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
        Region { size }
    }
}
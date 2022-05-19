use std::fmt;

#[derive(Debug)]
pub struct Prop {
    pub value: f32,
    pub sum: f32,
    pub sum2: f32,
}

impl Prop {
    pub fn new() -> Prop {
        Prop { value: 0_f32, sum: 0_f32, sum2: 0_f32 }
    }

    pub fn reset(&mut self) {
        self.value = 0_f32;
        self.sum = 0_f32;
        self.sum2 = 0_f32;
    }

    pub fn accumulate(&mut self) {
        self.sum += self.value;
        self.sum2 += self.value * self.value;
    }

    pub fn average(&mut self, steps: usize) {
        self.sum /= steps as f32;
        let dispersion = self.sum2 / (steps as f32) - self.sum * self.sum;
        self.sum2 = if dispersion > 0_f32 { dispersion.sqrt() } else { 0_f32 };
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}--{}", self.sum, self.sum2)
    }
}
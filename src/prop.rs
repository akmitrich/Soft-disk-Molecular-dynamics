#[derive(Debug)]
pub struct Prop {
    value: f32,
    sum: f32,
    sum2: f32,
}

impl Prop {
    pub fn new() -> Prop {
        Prop { value: 0_f32, sum: 0_f32, sum2: 0_f32 }
    }
}
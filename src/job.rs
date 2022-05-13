use crate::vector::{Vector, Region};

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: Vec<Vector<D>>,
    vel: Vec<Vector<D>>,
    acc: Vec<Vector<D>>,
    region: Region,
    delta_t: f32,
    density: f32,
    r_cut: f32,
    temperature: f32,
    t_now: f32,
    vel_magnitude: f32,
    u_sum: f32,
    vir_sum: f32,
    vv_sum: f32,
    more_cycles: bool,
    step_avg: usize,
    step_count: usize,
    step_equil: usize,
    step_limit: usize,
}

impl<const D: usize> Job<D> {
    pub fn setup_job() -> Job<D> {
        let mut result = Job { 
            pos: vec![],
            vel: vec![],
            acc: vec![],
            region: Region::new([10., 10., 10.])
        };
        result
    }
}
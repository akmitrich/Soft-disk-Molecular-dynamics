use crate::vector::{Vector, Region};
use crate::prop::Prop;

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: Vec<Vector<D>>,
    vel: Vec<Vector<D>>,
    acc: Vec<Vector<D>>,
    region: Region<D>,
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
    kin_energy: Prop,
    tot_energy: Prop,
    pressure: Prop,
}

impl<const D: usize> Job<D> {
    pub fn setup_job() -> Job<D> {
        let mut result: Job<D> = Job { 
            pos: vec![],
            vel: vec![],
            acc: vec![],
            region: Region::new([50.; D]),
            delta_t: 1e-3,
            density: 0.5,
            r_cut: 10_f32,
            temperature: 0_f32,
            t_now: 0_f32,
            vel_magnitude: 0_f32,
            u_sum: 0_f32,
            vir_sum: 0_f32,
            vv_sum: 0_f32,
            more_cycles: true,
            step_avg: 1,
            step_count: 0,
            step_equil: 0,
            step_limit: 10,
            kin_energy: Prop::new(),
            tot_energy: Prop::new(),
            pressure: Prop::new(),
        };
        result
    }
}
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
        result.init_coord();
        result.init_vels();
        result.init_acc();
        result
    }

    fn init_coord(&mut self) {
        let mut x = [0_f32; D];
        x[0] = 1_f32;
        self.pos.push(Vector::<D>::from(x));
        let mut x = [0_f32; D];
        x[0] = -1_f32;
        self.pos.push(Vector::<D>::from(x));
    }

    fn init_vels(&mut self) {
        let mut v = [0_f32; D];
        v[0] = -1_f32;
        self.vel.push(Vector::<D>::from(v));
        let mut v = [0_f32; D];
        v[0] = 1_f32;
        self.vel.push(Vector::<D>::from(v));
    }

    fn init_acc(&mut self) {
        for _ in 0..self.n_mol() {
            self.acc.push(Vector::<D>::new());
        }
    }

    pub fn n_mol(&self) -> usize {
        self.pos.len()
    }

    pub fn run(&mut self) {
        while self.more_cycles {
            self.single_step();
            if self.step_count >= self.step_limit {
                self.more_cycles = false;
            }
        }
    }

    fn single_step(&mut self) {
        self.step_count += 1;
        self.t_now = (self.step_count as f32) * self.delta_t;
        self.leapfrog_begin();
        self.apply_boundary_conditions();
        self.leapfrog_end();
    }

    fn apply_boundary_conditions(&mut self) {
        for i in 0..self.n_mol() {
            self.region.wrap(&mut self.pos[i]);
        }
    }

    fn leapfrog_begin(&mut self) {
        for i in 0..self.n_mol() {
            self.vel[i].plus(&Vector::from_scaled(&self.acc[i], self.delta_t / 2_f32));
            self.pos[i].plus(&Vector::from_scaled(&self.vel[i], self.delta_t));
        }
    }

    fn leapfrog_end(&mut self) {
        for i in 0..self.n_mol() {
            self.vel[i].plus(&Vector::from_scaled(&self.acc[i], self.delta_t / 2_f32));
        }
    }
}
use crate::vector::{Vector, Region};
use crate::prop::Prop;

//pub static global: Box<Job<3>> = Box::new(Job::<3>::setup_job());

#[derive(Debug)]
pub struct Job<const D: usize> {
    pos: Vec<Vector<D>>,
    vel: Vec<Vector<D>>,
    acc: Vec<Vector<D>>,
    boundary: Region<D>,
    delta_t: f32,
    density: f32,
    cut_off_radius: f32,
    temperature: f32,
    t_now: f32,
    vel_magnitude: f32,
    v_sum: Vector<D>,
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

pub struct JobSetup<const D: usize> {
    job: Job<D>,
}

impl<const D: usize> Default for Job<D> {
    fn default() -> Self {
        Job { 
            pos: vec![],
            vel: vec![],
            acc: vec![],
            boundary: Region::new([50.; D]),
            delta_t: 1e-3,
            density: 0.5,
            cut_off_radius: 11.5,
            temperature: 0_f32,
            t_now: 0_f32,
            vel_magnitude: 0_f32,
            v_sum: Vector::<D>::new(),
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
        }
    }
}

impl<const D: usize> Job<D> {
    fn reset_acc(&mut self) {
        self.acc.clear();
        for _ in 0..self.n_mol() {
            self.acc.push(Vector::<D>::new());
        }
    }

    fn reset_props(&mut self) {
        self.tot_energy.reset();
        self.kin_energy.reset();
        self.pressure.reset();
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
        self.compute_forces();
        self.leapfrog_end();
        self.evaluate_properties();
        self.accumulate_properties();
        if (self.step_count % self.step_avg == 0) {
            self.average_properies();
            self.print_summary();
            self.reset_props();
        }
    }

    fn apply_boundary_conditions(&mut self) {
        for i in 0..self.n_mol() {
            self.boundary.wrap(&mut self.pos[i]);
        }
    }

    fn leapfrog_begin(&mut self) {
        for i in 0..self.n_mol() {
            self.vel[i].plus(&self.acc[i].new_scaled_by(self.delta_t / 2_f32));
            self.pos[i].plus(&self.vel[i].new_scaled_by(self.delta_t));
        }
    }

    fn leapfrog_end(&mut self) {
        for i in 0..self.n_mol() {
            self.vel[i].plus(&self.acc[i].new_scaled_by(self.delta_t / 2_f32));
        }
    }

    fn compute_forces(&mut self) {
        if self.acc.len() == 0 {
            eprintln!("{:?} has nothing to do", self);
            return;
        }
        self.reset_acc();
        self.u_sum = 0_f32;
        self.vir_sum = 0_f32;
        let cut_off_squared = self.cut_off_radius * self.cut_off_radius;
        for i in 0..self.n_mol() - 1 {
            for j in (i + 1)..self.n_mol() {
                let mut distance_vector = Vector::difference(&self.pos[i], &self.pos[j]);
                self.boundary.wrap(&mut distance_vector);
                let distance_squared = distance_vector.vector_squared();
                if distance_squared < cut_off_squared {
                    let distance_squared_inverted = 1_f32 / distance_squared;
                    let distance_inverted_in_6th = distance_squared_inverted * distance_squared_inverted * distance_squared_inverted;
                    let force_value = 48_f32 * distance_inverted_in_6th * (distance_inverted_in_6th - 0.5) * distance_squared_inverted;
                    self.acc[i].plus(&distance_vector.new_scaled_by(force_value));
                    self.acc[j].plus(&distance_vector.new_scaled_by(-force_value));
                    self.u_sum += 4_f32 * distance_inverted_in_6th * (distance_inverted_in_6th - 1_f32) + 1_f32;
                    self.vir_sum += force_value * distance_squared;
                }
            }
        }
    }

    fn evaluate_properties(&mut self) {
        self.v_sum = Vector::<D>::new();
        self.vv_sum = 0_f32;
        for velocity in self.vel.iter() {
            self.v_sum.plus(velocity);
            let vv = velocity.squared_length();
            self.vv_sum += vv;
        }
        self.kin_energy.value = self.vv_sum / ((2 * self.n_mol()) as f32);
        self.tot_energy.value = self.kin_energy.value + self.u_sum / self.n_mol() as f32;
        self.pressure.value = self.density * (self.vv_sum + self.vir_sum) / ((self.n_mol() * D) as f32);
    }

    fn accumulate_properties(&mut self) {
        self.tot_energy.accumulate();
        self.kin_energy.accumulate();
        self.pressure.accumulate();
    }

    fn average_properies(&mut self) {
        self.tot_energy.average(self.step_avg);
        self.kin_energy.average(self.step_avg);
        self.pressure.average(self.step_avg);
    }

    fn print_summary(&self) {
        println!("stepCount = {}; timeNow = {}; average velocity component sum {}; total energy {}; kinetic energy {}; pressure {}.",
    self.step_count, self.t_now, self.v_sum.component_sum() / self.n_mol() as f32, self.tot_energy, self.kin_energy, self.pressure);
    }
}

impl<const D: usize> JobSetup<D> {
    pub fn start() -> Self {
        let mut result = Self {
            job: Job::default(),
        };
        result.trivial_pos();
        result.trivial_vels();
        result.job.reset_acc();
        result
    }

    fn trivial_pos(&mut self) {
        let mut x = [0_f32; D];
        x[0] = 1_f32;
        self.job.pos.push(Vector::<D>::from(x));
        let mut x = [0_f32; D];
        x[0] = -1_f32;
        self.job.pos.push(Vector::<D>::from(x));
    }

    fn trivial_vels(&mut self) {
        let mut v = [0_f32; D];
        v[0] = -1_f32;
        self.job.vel.push(Vector::<D>::from(v));
        let mut v = [0_f32; D];
        v[0] = 1_f32;
        self.job.vel.push(Vector::<D>::from(v));
    }


    pub fn step_avg(mut self, avg: usize) -> Self {
        self.job.step_avg = avg;
        self
    }

    pub fn step_limit(mut self, limit: usize) -> Self {
        self.job.step_limit = limit;
        self
    }

    pub fn region(mut self, bounds: Region<D>) -> Self {
        self.job.boundary = bounds;
        self
    }

    pub fn init_coords(mut self, cells: [usize; D]) -> Self {
        let mut item_number: usize = 1;
        for cell in cells.iter() {
            item_number *= cell;
        }
        assert!(item_number > 0 && item_number < 1_000_000);
        self.job.pos = Vec::with_capacity(item_number);
        let mut gap = [0_f32; D];
        for i in 0..D {
            gap[i] = self.job.boundary.size.components()[i] / cells[i] as f32;
        }
        let mut lattice = vec![];
        Self::lattice(&cells, &gap, &mut lattice, [0_f32; D], 0);
        let shift = self.job.boundary.size.new_scaled_by(-0.5);
        for node in lattice {
            let mut pos = Vector::<D>::from(node);
            pos.plus(&shift);
            self.job.pos.push(pos);
        }
        self
    }

    fn lattice(cells: &[usize; D], gap: &[f32; D], nodes: &mut Vec<[f32; D]>, mut current: [f32; D], current_index: usize) {
        for i in 0..cells[current_index] {
            current[current_index] = (0.5_f32 + i as f32) * gap[current_index];
            if current_index == D - 1 {
                nodes.push(current.clone());
            } else {
                Self::lattice(cells, gap, nodes, current, current_index + 1)
            }
        }
    }

    pub fn get_job(self) -> Job<D> {
        self.job
    }
}
mod prop;
mod vector;
mod job;
mod tests;

use job::JobSetup;

fn main() {
    let mut global = JobSetup::<3>::new()
        .step_limit(100)
        .step_avg(10)
        .region(vector::Region::new([30., 30., 30.]))
        .trivial()
        .lattice_and_random_vels([10,1,1], 1_f32)
        .get_job();
    global.run();
    println!("Mol:\n {:?}", global);
    print_forces();
}

fn print_forces() {
    let mut distance = 0_f32;
    loop {
        distance += 1e-2;
        let distance_squared = distance * distance;
        let distance_squared_inverted = 1_f32 / distance_squared;
        let distance_inverted_in_6th = distance_squared_inverted * distance_squared_inverted * distance_squared_inverted;
        let force_value = 48_f32 * distance_inverted_in_6th * (distance_inverted_in_6th - 0.5) * distance_squared_inverted;
        println!("distance = {}; force = {}", distance, force_value);
        if distance > 10_f32 {
            break;
        }
    }
}
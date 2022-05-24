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
//        .lattice_and_random_vels([10,1,1])
        .get_job();
    global.run();
    println!("Mol:\n {:?}", global);
}

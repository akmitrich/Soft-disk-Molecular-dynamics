mod prop;
mod vector;
mod job;

use job::JobSetup;

fn main() {
    let mut global = JobSetup::<3>::start()
        .step_limit(100)
        .step_avg(10)
        .region(vector::Region::new([30., 30., 30.]))
        .init_coords([4,4,4])
        .get_job();
 //   global.run();
    println!("Mol:\n {:?}", global);
}

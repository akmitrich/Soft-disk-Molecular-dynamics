mod prop;
mod vector;
mod job;

fn main() {
    let mut global = job::Job::<3>::setup_job();
    global.run();
    println!("Mol:\n {:?}", global);
}

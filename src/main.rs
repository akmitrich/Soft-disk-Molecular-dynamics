mod prop;
mod vector;
mod job;

fn main() {
    let mut global = job::Job::<3>::setup_job();
    global.run();
    println!("Mol:\n {:?}", global);
    for i in 0..100 {
        let d = (i + 1) as f32 * 0.1;
        println!("d = {} -> f = {}", d, force(d));
    }
}

fn force(d: f32) -> f32 {
    let distance_squared = d * d;
    let distance_squared_inverted = 1_f32 / distance_squared;
    let distance_inverted_in_6th = distance_squared_inverted * distance_squared_inverted * distance_squared_inverted;
    let force_value = 48_f32 * distance_inverted_in_6th * (distance_inverted_in_6th - 0.5) * distance_squared_inverted;
    force_value
}

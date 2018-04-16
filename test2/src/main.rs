use std::thread;
use std::cmp::min;

const NUMBER_OF_THREADS : u8 = 10; //0 - 255



fn main() {
    use std::time::Instant;
    let now = Instant::now();

    println!("Starting execution");

    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
    println!("Seconds passed: {}", sec);
}

use std::time::Instant;
use threads::sum;

fn main() {
    let v = (1..1e9 as i32).into_iter().collect();
    let now = Instant::now();
    let s = sum(v);
    let elapsed_time = now.elapsed();
    println!(
        "Running slow_function() took {} seconds.",
        elapsed_time.as_secs()
    );
}

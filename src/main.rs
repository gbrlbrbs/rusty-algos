use rusty_algos::sorting::merge_sort;
use rand::{distributions::{Distribution, Uniform}, Rng};

fn main() {
    let dist = Uniform::new(-100_000, 100_000);
    let rng = rand::thread_rng();
    let values: Vec<i64> = rng.sample_iter(&dist).take(100_000).collect();

}

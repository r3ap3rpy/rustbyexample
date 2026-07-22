use rayon::prelude::*;
use std::time::Instant;
use rand::random_range;
fn main() {
    let first_start = Instant::now();
    let mut nums: Vec<i64> = (0..1000_000_000).map(|v| v + 1000).collect();
    nums.par_iter_mut().for_each(|n| {*n * 3;});
    dbg!(first_start.elapsed());

    let second_start = Instant::now();
    let mut numz: Vec<i64> = (0..1000_000_000).map(|v| v + 1000).collect();
    numz.iter_mut().for_each(|n|{*n * 3;});
    dbg!(second_start.elapsed());

    let sumstart = Instant::now();
    let sum: u64 = (0..1000_000).into_par_iter().map(|n|{n * random_range(0..1000)}).sum();
    dbg!(sumstart.elapsed());
}

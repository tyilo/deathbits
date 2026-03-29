use std::time::Duration;

use deathbits::{DiceSumOutcomes, FromRatio, Num};

fn main() {
    divan::main();
}

#[divan::bench(
    types = [
        f64,
        num_bigint::BigUint,
        arpfloat::Float,
        fast_posit::p64,
        fast_posit::Posit<64, 20, i64>,
        deathbits::LogNum<f32>,
        deathbits::LogNum<f64>,
    ],
    args = [5, 10, 15],
    max_time = Duration::from_secs(10),
)]
fn bench<T: Num>(n: u32) -> Vec<f64> {
    let mut cache = DiceSumOutcomes::<T>::new();
    cache
        .deathbit_stats(n)
        .into_iter()
        .map(|v| v.as_f64())
        .collect()
}

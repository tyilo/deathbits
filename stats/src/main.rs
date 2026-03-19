#![recursion_limit = "1000"]

use std::fmt::Display;

use deathbits::{DiceSumOutcomes, FromRatio, Num, dice_needed, ilog, total_outcomes};
use itertools::Itertools;
use num_bigint::BigUint;

struct DisplayApprox<'a>(&'a BigUint);

impl Display for DisplayApprox<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log10 = ilog(self.0, &10u8.into());
        let s = if f.width().is_some_and(|width| log10 < width.into()) {
            format!("{}", self.0)
        } else {
            format!("~10^{log10}")
        };
        f.pad(&s)
    }
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn run<T: Num>() {
    let mut cache = DiceSumOutcomes::<T>::new();
    for n in 1.. {
        let dice = dice_needed(n);
        let outcomes = total_outcomes(dice);
        let stats: Vec<_> = cache
            .deathbit_stats(n)
            .into_iter()
            .map(|v| v.as_f64())
            .collect();
        let end_pattern: Vec<_> = stats
            .iter()
            .copied()
            .rev()
            .take_while(|v| !(0.01..=0.99).contains(v))
            .map(|v| v.round() as u8)
            .collect();

        println!(
            "n={n:<2} k={dice:<5} outcomes={:<9} {}",
            DisplayApprox(&outcomes),
            stats.into_iter().map(|v| format!("{v:.02}")).join(", ")
        );
        println!("  Pattern: {end_pattern:?}");
    }
}

fn main() {
    //run::<f64>();
    //run::<BigUint>();
    //run::<arpfloat::Float>();
    //run::<fast_posit::p64>();
    //run::<fast_posit::Posit<64, 20, i64>>();
    run::<custom_float::ieee754::FpHalf>();
}

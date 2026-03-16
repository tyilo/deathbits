use std::fmt::Display;

use deathbits::{DiceSumOutcomes, dice_needed, ilog, total_outcomes};
use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::ToPrimitive;

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

fn main() {
    let mut cache = DiceSumOutcomes::new();
    for n in 1..20 {
        let dice = dice_needed(n);
        let outcomes = total_outcomes(dice);
        println!(
            "n={n:<2} k={dice:<4} outcomes={:<8} {}",
            DisplayApprox(&outcomes),
            cache
                .deathbit_stats(n)
                .into_iter()
                .map(|v| format!("{:.02}", v.to_f64().unwrap()))
                .join(", ")
        );
    }
}

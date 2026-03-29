use std::io::Write;
use std::{any::type_name, fmt::Display, fs::File, io};

use deathbits::{DiceSumOutcomes, FromRatio, Num, dice_needed, ilog, total_outcomes};
use itertools::Itertools;
use num_bigint::BigUint;
use tee_readwrite::TeeWriter;

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
    let stdout = io::stdout().lock();
    let log_file = File::create(format!("output/{}.txt", type_name::<T>())).unwrap();
    let mut writer = TeeWriter::new(stdout, log_file);

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

        writeln!(
            writer,
            "n={n:<2} k={dice:<5} outcomes={:<9} {}",
            DisplayApprox(&outcomes),
            stats.into_iter().map(|v| format!("{v:.02}")).join(", ")
        )
        .unwrap();
        writeln!(writer, "  Pattern: {end_pattern:?}").unwrap();
        writer.flush().unwrap();
    }
}

fn main() {
    //run::<f64>();
    //run::<BigUint>();
    //run::<arpfloat::Float>();
    //run::<fast_posit::p64>();
    //run::<fast_posit::Posit<64, 20, i64>>();
    run::<deathbits::LogNum<f64>>();
}

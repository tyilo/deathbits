use std::sync::LazyLock;

use num_bigint::BigUint;
use num_rational::Ratio;
use polonius_the_crab::{polonius, polonius_return};

const EYES: usize = 6;

#[must_use]
pub fn dice_needed(players: u32) -> usize {
    2usize.pow(players - 1).div_ceil(EYES)
}

/// # Panics
/// Panics if `dice` can't fit in a `u32`
#[must_use]
pub fn total_outcomes(dice: usize) -> BigUint {
    BigUint::from(EYES).pow(dice.try_into().unwrap())
}

/// ```
/// use deathbits::ilog;
///
/// assert_eq!(ilog(&0u8.into(), &10u8.into()), 0u8.into());
/// assert_eq!(ilog(&1u8.into(), &10u8.into()), 0u8.into());
/// assert_eq!(ilog(&9u8.into(), &10u8.into()), 0u8.into());
/// assert_eq!(ilog(&10u8.into(), &10u8.into()), 1u8.into());
/// assert_eq!(ilog(&11u8.into(), &10u8.into()), 1u8.into());
/// assert_eq!(ilog(&99u8.into(), &10u8.into()), 1u8.into());
/// assert_eq!(ilog(&100u8.into(), &10u8.into()), 2u8.into());
/// ```
#[must_use]
pub fn ilog(a: &BigUint, b: &BigUint) -> BigUint {
    let mut rem = a.clone();
    let mut res = BigUint::ZERO;
    while rem >= *b {
        res += BigUint::from(1u8);
        rem /= b;
    }
    res
}

#[derive(Default)]
pub struct DiceSumOutcomes {
    cache: Vec<Vec<BigUint>>,
}

impl DiceSumOutcomes {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    fn get_row(&mut self, dice: usize) -> &[BigUint] {
        static ZERO_ROW: LazyLock<[BigUint; 1]> = LazyLock::new(|| [1u8.into()]);

        let Some(row_index) = dice.checked_sub(1) else {
            return &*ZERO_ROW;
        };

        let mut this = self;
        polonius!(|this| -> &'polonius [BigUint] {
            if let Some(row) = this.cache.get(row_index) {
                polonius_return!(row);
            }
        });

        let prev_row = this.get_row(row_index);
        let offset = dice - 1;
        let mut row = vec![];
        for sum in dice..=EYES * dice {
            let mut count = BigUint::ZERO;
            for i in 1..=EYES {
                let Some(j) = sum.checked_sub(i).and_then(|v| v.checked_sub(offset)) else {
                    continue;
                };
                if let Some(v) = prev_row.get(j) {
                    count += v;
                }
            }
            row.push(count);
        }

        this.cache.push(row);
        this.cache.last().unwrap()
    }

    #[must_use]
    fn get_aux(&mut self, dice: usize, sum: usize) -> Option<&BigUint> {
        self.get_row(dice).get(sum.checked_sub(dice)?)
    }

    #[must_use]
    pub fn get(&mut self, dice: usize, sum: usize) -> &BigUint {
        const ZERO: &BigUint = &BigUint::ZERO;
        self.get_aux(dice, sum).unwrap_or(ZERO)
    }

    /// # Panics
    /// Pancis if `players` can't fit into a `usize`
    #[must_use]
    pub fn deathbit_stats(&mut self, players: u32) -> Vec<Ratio<BigUint>> {
        let dice = 2usize.pow(players - 1).div_ceil(EYES);
        let row = self.get_row(dice);
        let mut counts = vec![BigUint::ZERO; players.try_into().unwrap()];
        for (i, v) in row.iter().enumerate() {
            let sum = dice + i;
            for (j, count) in counts.iter_mut().enumerate() {
                if sum & (1 << j) != 0 {
                    *count += v;
                }
            }
        }
        let total = total_outcomes(dice);
        counts
            .into_iter()
            .map(|count| Ratio::new(count, total.clone()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rows() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get_row(0), [1u8].map(Into::into));
        assert_eq!(
            cache.get_row(1),
            [1u8, 1u8, 1u8, 1u8, 1u8, 1u8].map(Into::into)
        );
        assert_eq!(
            cache.get_row(2),
            [1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 5u8, 4u8, 3u8, 2u8, 1u8].map(Into::into)
        );
    }

    #[test]
    fn row_sums() {
        let mut cache = DiceSumOutcomes::new();
        for dice in 0..100 {
            let sum: BigUint = cache.get_row(dice).iter().sum();
            assert_eq!(sum, total_outcomes(dice));
        }
    }

    #[test]
    fn get_2_0() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 0), &0u8.into());
    }

    #[test]
    fn get_2_1() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 1), &0u8.into());
    }

    #[test]
    fn get_2_2() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 2), &1u8.into());
    }

    #[test]
    fn get_2_7() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 7), &6u8.into());
    }

    #[test]
    fn get_2_12() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 12), &1u8.into());
    }

    #[test]
    fn get_2_13() {
        let mut cache = DiceSumOutcomes::new();
        assert_eq!(cache.get(2, 13), &0u8.into());
    }
}

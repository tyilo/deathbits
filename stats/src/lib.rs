#![feature(gen_blocks)]
// For custom_float:
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::ops::AddAssign;

use arpfloat::{Float, RoundingMode, Semantics};
use custom_float::{Fp, UInt, bitsize_of};
use fast_posit::{Posit, RoundInto};
use num_bigint::BigUint;
use num_rational::Ratio;
use num_traits::ToPrimitive;
use polonius_the_crab::{polonius, polonius_return};

const EYES: usize = 6;

#[must_use]
pub fn dice_needed(players: u32) -> usize {
    2usize.pow(players - 1).div_ceil(EYES)
}

/// # Panics
/// Panics if `dice` can't fit in a `u32`
#[must_use]
pub fn total_outcomes<T: Num>(dice: usize) -> T {
    T::from_u64(EYES.try_into().unwrap()).pow(dice.try_into().unwrap())
}

pub trait FromRatio<T> {
    #[must_use]
    fn from_ratio(n: T, d: T) -> Self;

    #[must_use]
    fn as_f64(&self) -> f64;
}

pub trait Num: Sized + Clone + for<'a> AddAssign<&'a Self> {
    type Ratio: FromRatio<Self>;

    #[must_use]
    fn from_u64(v: u64) -> Self;

    #[must_use]
    fn pow(&self, v: u32) -> Self;

    #[must_use]
    fn zero() -> Self {
        Self::from_u64(0)
    }

    #[must_use]
    fn one() -> Self {
        Self::from_u64(1)
    }
}

impl FromRatio<f64> for f64 {
    fn from_ratio(n: f64, d: f64) -> Self {
        n / d
    }

    fn as_f64(&self) -> f64 {
        *self
    }
}

impl Num for f64 {
    type Ratio = Self;

    #[allow(clippy::cast_precision_loss)]
    fn from_u64(v: u64) -> Self {
        v as f64
    }

    fn pow(&self, v: u32) -> Self {
        self.powf(v.into())
    }
}

impl FromRatio<BigUint> for Ratio<BigUint> {
    fn from_ratio(n: BigUint, d: BigUint) -> Self {
        Self::new(n, d)
    }

    fn as_f64(&self) -> f64 {
        self.to_f64().unwrap()
    }
}

impl Num for BigUint {
    type Ratio = Ratio<BigUint>;

    fn from_u64(v: u64) -> Self {
        v.into()
    }

    fn pow(&self, v: u32) -> Self {
        self.pow(v)
    }
}

const SEMANTICS: Semantics = Semantics::new(20, 20, RoundingMode::NearestTiesToEven);

impl FromRatio<Float> for Float {
    fn from_ratio(n: Float, d: Float) -> Self {
        n / d
    }

    fn as_f64(&self) -> f64 {
        self.as_f64()
    }
}

impl Num for Float {
    type Ratio = Self;

    fn from_u64(v: u64) -> Self {
        Self::from_u64(SEMANTICS, v)
    }

    fn pow(&self, v: u32) -> Self {
        self.pow(&Self::from_u64(SEMANTICS, v.into()))
    }
}

impl<const N: u32, const ES: u32, Int: fast_posit::Int> FromRatio<Posit<N, ES, Int>>
    for Posit<N, ES, Int>
{
    fn from_ratio(n: Posit<N, ES, Int>, d: Posit<N, ES, Int>) -> Self {
        n / d
    }

    fn as_f64(&self) -> f64 {
        // Fix unimplemented case in `fast_posit`
        if self < &f64::MIN_POSITIVE.round_into() {
            return 0.0;
        }
        (*self).round_into()
    }
}

impl<const N: u32, const ES: u32, Int: fast_posit::Int> Num for Posit<N, ES, Int> {
    type Ratio = Self;

    fn from_u64(v: u64) -> Self {
        i128::from(v).round_into()
    }

    /// ```
    /// use deathbits::Num;
    /// use fast_posit::{p64, RoundFrom};
    ///
    /// let two = p64::round_from(2i64);
    /// for i in 0..10 {
    ///     assert_eq!(two.pow(i), p64::round_from(2i64.pow(i)), "2^{i}: {}", f64::round_from(two.pow(i)));
    /// }
    /// ```
    fn pow(&self, mut v: u32) -> Self {
        let mut base = *self;
        let mut res = Self::one();
        while v > 0 {
            if v % 2 == 1 {
                res *= base;
            }
            v /= 2;
            base *= base;
        }
        res
    }
}

impl<
    U: UInt,
    const SIGN_BIT: bool,
    const EXP_SIZE: usize,
    const INT_SIZE: usize,
    const FRAC_SIZE: usize,
    const EXP_BASE: usize,
> FromRatio<Fp<U, SIGN_BIT, EXP_SIZE, INT_SIZE, FRAC_SIZE, EXP_BASE>>
    for Fp<U, SIGN_BIT, EXP_SIZE, INT_SIZE, FRAC_SIZE, EXP_BASE>
where
    [(); bitsize_of::<U>() - SIGN_BIT as usize - EXP_SIZE - INT_SIZE - FRAC_SIZE]:,
    [(); EXP_BASE - 2]:,
{
    fn from_ratio(
        n: Fp<U, SIGN_BIT, EXP_SIZE, INT_SIZE, FRAC_SIZE, EXP_BASE>,
        d: Fp<U, SIGN_BIT, EXP_SIZE, INT_SIZE, FRAC_SIZE, EXP_BASE>,
    ) -> Self {
        n / d
    }

    fn as_f64(&self) -> f64 {
        (*self).into()
    }
}

impl<
    U: UInt,
    const SIGN_BIT: bool,
    const EXP_SIZE: usize,
    const INT_SIZE: usize,
    const FRAC_SIZE: usize,
    const EXP_BASE: usize,
> Num for Fp<U, SIGN_BIT, EXP_SIZE, INT_SIZE, FRAC_SIZE, EXP_BASE>
where
    [(); bitsize_of::<U>() - SIGN_BIT as usize - EXP_SIZE - INT_SIZE - FRAC_SIZE]:,
    [(); EXP_BASE - 2]:,
{
    type Ratio = Self;

    fn from_u64(v: u64) -> Self {
        v.into()
    }

    fn pow(&self, v: u32) -> Self {
        self.powi(i64::from(v))
    }
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

trait ZigZag: DoubleEndedIterator {
    fn zig_zag(self) -> impl Iterator<Item = Self::Item>;
}

impl<T: DoubleEndedIterator> ZigZag for T {
    fn zig_zag(mut self) -> impl Iterator<Item = Self::Item> {
        gen move {
            loop {
                match self.next() {
                    None => break,
                    Some(v) => yield v,
                }
                match self.next_back() {
                    None => break,
                    Some(v) => yield v,
                }
            }
        }
    }
}

pub struct DiceSumOutcomes<T> {
    zero: T,
    cache: Vec<Vec<T>>,
}

impl<T: Num> Default for DiceSumOutcomes<T> {
    fn default() -> Self {
        let zero_row = vec![T::one()];
        Self {
            zero: T::zero(),
            cache: vec![zero_row],
        }
    }
}

impl<T: Num> DiceSumOutcomes<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    fn get_row(&mut self, dice: usize) -> &[T] {
        let mut this = self;
        polonius!(|this| -> &'polonius [T] {
            if let Some(row) = this.cache.get(dice) {
                polonius_return!(row);
            }
        });

        let prev_row = this.get_row(dice - 1);
        let offset = dice - 1;
        let mut row = vec![];
        for sum in dice..=EYES * dice {
            let mut count = T::zero();
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
    fn get_aux(&mut self, dice: usize, sum: usize) -> Option<&T> {
        self.get_row(dice).get(sum.checked_sub(dice)?)
    }

    #[must_use]
    pub fn get(&mut self, dice: usize, sum: usize) -> &T {
        let mut this = self;
        polonius!(|this| -> &'polonius T {
            if let Some(v) = this.get_aux(dice, sum) {
                polonius_return!(v);
            }
        });
        &this.zero
    }

    /// # Panics
    /// Pancis if `players` can't fit into a `usize`
    #[must_use]
    pub fn deathbit_stats(&mut self, players: u32) -> Vec<T::Ratio> {
        let dice = 2usize.pow(players - 1).div_ceil(EYES);
        let row = self.get_row(dice);
        let mut counts = vec![T::zero(); players.try_into().unwrap()];
        for (i, v) in row.iter().enumerate().zig_zag() {
            let sum = dice + i;
            for (j, count) in counts.iter_mut().enumerate() {
                if sum & (1 << j) != 0 {
                    *count += v;
                }
            }
        }
        let total: T = total_outcomes(dice);
        counts
            .into_iter()
            .map(|count| T::Ratio::from_ratio(count, total.clone()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn biguint_cache() -> DiceSumOutcomes<BigUint> {
        DiceSumOutcomes::new()
    }

    #[test]
    fn rows() {
        let mut cache = biguint_cache();
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
        let mut cache = biguint_cache();
        for dice in 0..100 {
            let sum: BigUint = cache.get_row(dice).iter().sum();
            assert_eq!(sum, total_outcomes(dice));
        }
    }

    #[test]
    fn get_2_0() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 0), &0u8.into());
    }

    #[test]
    fn get_2_1() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 1), &0u8.into());
    }

    #[test]
    fn get_2_2() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 2), &1u8.into());
    }

    #[test]
    fn get_2_7() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 7), &6u8.into());
    }

    #[test]
    fn get_2_12() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 12), &1u8.into());
    }

    #[test]
    fn get_2_13() {
        let mut cache = biguint_cache();
        assert_eq!(cache.get(2, 13), &0u8.into());
    }
}

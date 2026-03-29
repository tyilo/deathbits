use std::{
    mem::swap,
    ops::{Add, AddAssign},
};

use std_traits::num::Unsigned;

/// A number of with the bit representation `<exp> | <frac>`
/// which represents the value `2^exp * (frac + bias) - bias`
/// where `bias = 2^FRAC_SIZE`.
#[derive(Clone, Copy)]
pub struct UIntFloat<T: Unsigned, const EXP_SIZE: u32, const FRAC_SIZE: u32> {
    raw: T,
}

impl<T: Unsigned, const EXP_SIZE: u32, const FRAC_SIZE: u32> UIntFloat<T, EXP_SIZE, FRAC_SIZE> {
    pub const fn from_raw(raw: T) -> Self {
        Self { raw }
    }

    pub const fn min() -> Self {
        Self::from_raw(T::ZERO)
    }

    pub const fn max() -> Self {
        Self::from_raw(T::MAX)
    }

    fn frac_mask() -> T {
        T::ONE
            .checked_shl(FRAC_SIZE)
            .unwrap_or(T::ZERO)
            .wrapping_sub(T::ONE)
    }

    pub fn frac(self) -> T {
        self.raw & Self::frac_mask()
    }

    pub fn exp(self) -> T {
        self.raw.checked_shr(FRAC_SIZE).unwrap_or(T::ZERO)
    }

    pub fn into_parts(self) -> (T, T) {
        (self.frac(), self.exp())
    }

    pub fn from_parts(frac: T, exp: T) -> Self {
        let raw = exp.checked_shl(FRAC_SIZE).unwrap_or(T::ZERO) | (frac & Self::frac_mask());
        Self::from_raw(raw)
    }

    /// ```
    /// use deathbits::uint_float::UIntFloat;
    ///
    /// type F = UIntFloat<u8, 4, 4>;
    ///
    /// assert_eq!(F::min().to_f64_approx(), 0.0);
    /// assert_eq!(F::max().to_f64_approx(), 1015792.0);
    /// ```
    pub fn to_f64_approx(self) -> f64 {
        let exp: i32 = match self.exp().try_into() {
            Ok(v) => v,
            Err(_) => return f64::INFINITY,
        };
        let bias = 2f64.powi(FRAC_SIZE as _);
        2f64.powi(exp) * (self.frac().cast_float::<f64>() + bias) - bias
    }
}

// Assume `e1 >= e2`, then:
// ```
// (2^e1 * (a1 + bias) - bias) + (2^e2 * (a2 + bias) - bias)`
// = 2^e1 * (a1 + bias + 2^(e1 - e2) + bias) - 2 bias
// ```
impl<T: Unsigned, const EXP_SIZE: u32, const FRAC_SIZE: u32> Add
    for UIntFloat<T, EXP_SIZE, FRAC_SIZE>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut a1, mut e1) = self.into_parts();
        let (mut a2, mut e2) = rhs.into_parts();

        if e1 > e2 {
            swap(&mut e1, &mut e2);
            swap(&mut a1, &mut a2);
        }
        let diff = (e2 - e1).try_into().unwrap_or(u32::MAX);
        let a1 = a1.checked_shr(diff).unwrap_or(T::ZERO);

        let mut e = e2;
        let (a, carry) = a1.overflowing_add(a2);
        if carry {
            e = e.saturating_add(T::ONE);
        }
        Self::from_parts(a, e)
    }
}

impl<T: Unsigned, const EXP_SIZE: u32, const FRAC_SIZE: u32> AddAssign<&Self>
    for UIntFloat<T, EXP_SIZE, FRAC_SIZE>
{
    fn add_assign(&mut self, rhs: &Self) {
        *self = *self + *rhs;
    }
}

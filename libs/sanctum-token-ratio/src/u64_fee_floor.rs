use crate::{AmtsAfterFee, MathError, U64RatioFloor};

/// A fee ratio that should be <= 1.0.
/// fee_amt = floor(amt * fee_num / fee_denom)
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct U64FeeFloor<N: Copy + Into<u128>, D: Copy + Into<u128>> {
    pub fee_num: N,
    pub fee_denom: D,
}

impl<N: Copy + Into<u128>, D: Copy + Into<u128>> U64FeeFloor<N, D> {
    /// Returns no fees charged if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if fee_num > fee_denom (fee > 100%)
    pub fn apply(&self, amt: u64) -> Result<AmtsAfterFee, MathError> {
        let fees_charged = U64RatioFloor {
            num: self.fee_num,
            denom: self.fee_denom,
        }
        .apply(amt)?;
        let amt_after_fee = amt.checked_sub(fees_charged).ok_or(MathError)?;
        Ok(AmtsAfterFee {
            amt_after_fee,
            fees_charged,
        })
    }

    /// Returns a possible amount that was fed into self.apply()
    ///
    /// Returns `amt_after_apply` if fee_num == 0 || fee_denom == 0
    ///
    /// Errors if fee_num > fee_denom (fee > 100%)
    pub fn pseudo_reverse(&self, amt_after_fee: u64) -> Result<u64, MathError> {
        let n = self.fee_num.into();
        let d = self.fee_denom.into();
        if n == 0 || d == 0 {
            return Ok(amt_after_fee);
        }
        if n >= d {
            return Err(MathError);
        }
        let y: u128 = amt_after_fee.into();
        let dy = y.checked_mul(d).ok_or(MathError)?;
        let d_minus_n = d - n; // n < d checked above
        let q_floor = dy / d_minus_n; // d_minus_n != 0 since d != n

        q_floor.try_into().map_err(|_e| MathError)
    }

    pub fn is_valid(&self) -> bool {
        self.fee_num.into() <= self.fee_denom.into()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn u64_fee_lte_one()
            (fee_denom in any::<u64>())
            (fee_num in 0..=fee_denom, fee_denom in Just(fee_denom)) -> U64FeeFloor<u64, u64> {
                U64FeeFloor { fee_num, fee_denom }
            }
    }

    proptest! {
        #[test]
        fn u64_fee_invariants(amt: u64, fee in u64_fee_lte_one()) {
            let AmtsAfterFee { amt_after_fee, fees_charged } = fee.apply(amt).unwrap();
            prop_assert!(amt_after_fee <= amt);
            prop_assert_eq!(amt, amt_after_fee + fees_charged);
        }
    }

    proptest! {
        #[test]
        fn u64_fee_round_trip(amt: u64, fee in u64_fee_lte_one()) {
            let AmtsAfterFee { amt_after_fee, .. } = fee.apply(amt).unwrap();

            let reversed = fee.pseudo_reverse(amt_after_fee).unwrap();
            let apply_on_reversed = fee.apply(reversed).unwrap();

            prop_assert_eq!(amt_after_fee, apply_on_reversed.amt_after_fee);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_denom(fee_num: u64, fee_denom in Just(0u64), amt: u64) {
            let fee = U64FeeFloor { fee_num, fee_denom };
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);

            let reversed = fee.pseudo_reverse(amt).unwrap();
            prop_assert_eq!(reversed, amt);
        }
    }

    proptest! {
        #[test]
        fn u64_zero_num(fee_num in Just(0u64), fee_denom: u64, amt: u64) {
            let fee = U64FeeFloor { fee_num, fee_denom };
            let amts_after_fee = fee.apply(amt).unwrap();

            prop_assert_eq!(amts_after_fee.amt_after_fee, amt);
            prop_assert_eq!(amts_after_fee.fees_charged, 0);

            let reversed = fee.pseudo_reverse(amt).unwrap();
            prop_assert_eq!(reversed, amt);
        }
    }

    prop_compose! {
        fn u64_smaller_larger()
            (boundary in any::<u64>())
            (smaller in 0..=boundary, larger in boundary..=u64::MAX) -> (u64, u64) {
                (smaller, larger)
            }
    }

    proptest! {
        #[test]
        fn valid_invalid((smaller, larger) in u64_smaller_larger()) {
            let valid = U64FeeFloor { fee_num: smaller, fee_denom: larger };
            prop_assert!(valid.is_valid());
            if smaller != larger {
                let invalid = U64FeeFloor { fee_num: larger, fee_denom: smaller };
                prop_assert!(!invalid.is_valid());
            }
        }
    }
}

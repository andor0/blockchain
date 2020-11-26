use crate::EraIndex;
use sp_runtime::{Perbill, Percent, SaturatedConversion, traits::{AtLeast32BitUnsigned, Saturating, Zero}};

/// The total payout to all validators (and their nominators) per era and maximum payout.
///
/// Defined as such:
/// `maximum-payout = 1.000233278 * total-tokens (1 - 0.00005) ^ era-index`
/// `staker-payout = maximum_payout * 0.7`
pub fn compute_total_payout<N>(
    era_index: EraIndex,
    total_tokens: N,
    total_issuance: N,
) -> (N, N) where N: AtLeast32BitUnsigned + Clone {
    if era_index < 360_000 {
        // If era < 360,000 mint according to inflation formula
        let inflation_rate = Perbill::from_rational_approximation(233_278u128, 1_000_000_000u128);
        let inflation_decay = Perbill::from_rational_approximation(999_950_000u128, 1_000_000_000u128)
            .saturating_pow(era_index.saturated_into());

        let staker_payout = inflation_rate.mul_ceil(inflation_decay.mul_ceil(total_tokens));
        let maximum_payout = inflation_rate.mul_ceil(inflation_decay.mul_ceil(total_issuance));

        let staker_to_treasury_ratio = Percent::from_rational_approximation(7u32, 10u32);
        let staker_maximum = staker_to_treasury_ratio.mul_floor(maximum_payout.clone());

        if staker_payout > staker_maximum {
            (staker_maximum, maximum_payout)
        } else {
            (staker_payout, maximum_payout)
        }
    } else if era_index == 360_000 {
        let maximum_payout = 7_777_777_777u128.saturated_into::<N>().saturating_sub(total_issuance);
        let staker_to_treasury_ratio = Percent::from_rational_approximation(7u32, 10u32);
        let staker_maximum = staker_to_treasury_ratio.mul_floor(maximum_payout.clone());
        (staker_maximum, maximum_payout)
    } else {
        // If era > 360,000 no more minting
        let maximum_payout = Zero::zero();
        let staker_payout = Zero::zero();
        (staker_payout, maximum_payout)
    }
}

#[cfg(test)]
mod test {
	#[test]
	fn calculation_is_sensible() {
        const TOTAL_TOKENS: u128 = 77_777_777;
        const TOTAL_ISSUANCE3: u128 = 77_777_777;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12700, 18144));
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12700, 18143));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12700, 18143));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12699, 18142));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12698, 18141));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12698, 18140));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12697, 18139));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12696, 18138));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12695, 18137));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12695, 18136));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12694, 18135));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12387, 17696));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (12081, 17259));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (7703, 11005));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (632, 904));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (86, 123));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (31, 45));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (2, 3));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (5390000000, 7700000000));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE3), (0, 0));

        const TOTAL_ISSUANCE4: u128 = 1_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18144, 233278));
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18143, 233267));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18143, 233255));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18142, 233244));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18141, 233232));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18140, 233220));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18139, 233209));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18138, 233197));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18137, 233185));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18136, 233174));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (18135, 233162));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (17696, 227519));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (17259, 221901));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (11005, 141488));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (904, 11612));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (123, 1570));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (45, 576));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (3, 27));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (4744444443, 6777777777));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE4), (0, 0));

        const TOTAL_ISSUANCE5: u128 = 10_000_000_000;

        assert_eq!(super::compute_total_payout(0u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18144, 2332780));
        assert_eq!(super::compute_total_payout(1u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18143, 2332664));
        assert_eq!(super::compute_total_payout(2u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18143, 2332547));
        assert_eq!(super::compute_total_payout(3u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18142, 2332431));
        assert_eq!(super::compute_total_payout(4u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18141, 2332314));
        assert_eq!(super::compute_total_payout(5u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18140, 2332197));
        assert_eq!(super::compute_total_payout(6u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18139, 2332081));
        assert_eq!(super::compute_total_payout(7u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18138, 2331964));
        assert_eq!(super::compute_total_payout(8u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18137, 2331848));
        assert_eq!(super::compute_total_payout(9u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18136, 2331731));
        assert_eq!(super::compute_total_payout(10u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (18135, 2331614));
        assert_eq!(super::compute_total_payout(500u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (17696, 2275182));
        assert_eq!(super::compute_total_payout(1_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (17259, 2219006));
        assert_eq!(super::compute_total_payout(10_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (11005, 1414876));
        assert_eq!(super::compute_total_payout(60_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (904, 116112));
        assert_eq!(super::compute_total_payout(100_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (123, 15694));
        assert_eq!(super::compute_total_payout(120_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (45, 5759));
        assert_eq!(super::compute_total_payout(180_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (3, 266));
        assert_eq!(super::compute_total_payout(240_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(300_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(360_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
        assert_eq!(super::compute_total_payout(500_000u32, TOTAL_TOKENS, TOTAL_ISSUANCE5), (0, 0));
    }

    #[test]
    fn total_issuance_should_grow_predictable() {
        const TOTAL_ISSUANCE: u128 = 77_777_777;
        const TOTAL_TOKENS: u128 = 77_777_777;

        assert_eq!(total_issuance_after_n_eras(1, TOTAL_TOKENS, TOTAL_ISSUANCE), 77795921); // vs 77795919
        assert_eq!(total_issuance_after_n_eras(10, TOTAL_TOKENS, TOTAL_ISSUANCE), 77959180); // vs 77959356
        assert_eq!(total_issuance_after_n_eras(20, TOTAL_TOKENS, TOTAL_ISSUANCE), 78140492); // vs 78141267
        assert_eq!(total_issuance_after_n_eras(30, TOTAL_TOKENS, TOTAL_ISSUANCE), 78321713); // vs 78323512
        assert_eq!(total_issuance_after_n_eras(40, TOTAL_TOKENS, TOTAL_ISSUANCE), 78502844); // vs 78506091
        assert_eq!(total_issuance_after_n_eras(50, TOTAL_TOKENS, TOTAL_ISSUANCE), 78683884); // vs 78689003
        assert_eq!(total_issuance_after_n_eras(60, TOTAL_TOKENS, TOTAL_ISSUANCE), 78864834); // vs 78872250
        assert_eq!(total_issuance_after_n_eras(70, TOTAL_TOKENS, TOTAL_ISSUANCE), 79045693); // vs 79055831
        assert_eq!(total_issuance_after_n_eras(80, TOTAL_TOKENS, TOTAL_ISSUANCE), 79226462); // vs 79239747
        assert_eq!(total_issuance_after_n_eras(90, TOTAL_TOKENS, TOTAL_ISSUANCE), 79407140); // vs 79423998
        assert_eq!(total_issuance_after_n_eras(100, TOTAL_TOKENS, TOTAL_ISSUANCE), 79587728); // vs 79608584
    }

    fn total_issuance_after_n_eras(n: super::EraIndex, total_tokens: u128, total_issuance: u128) -> u128 {
        (0..n).fold(total_issuance, |mut acc, era| {
            acc += super::compute_total_payout(era, total_tokens, total_issuance).1;
            acc
        })
    }
}
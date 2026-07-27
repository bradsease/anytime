use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCG, TT, UT1, UTC};
use crate::{Scale, Time};

const J2000_JD: f64 = 2_451_545.0;
const DAYS_PER_JULIAN_MILLENNIUM: f64 = 365_250.0;
const MICROSECONDS_PER_SECOND: f64 = 1_000_000.0;

#[derive(Debug)]
struct Term {
    power: usize,
    amplitude: f64,
    frequency: f64,
    phase: f64,
}

include!(concat!(env!("OUT_DIR"), "/fb2001.rs"));

fn terms() -> &'static [Term] {
    TERMS
}

fn tdb_minus_tt_seconds(tt: &Time<TT>) -> f64 {
    let (jd1, jd2) = tt.split_jd();
    let millennia = ((jd1 - J2000_JD) + jd2) / DAYS_PER_JULIAN_MILLENNIUM;
    let mut coefficients = [0.0; 6];

    for term in terms() {
        coefficients[term.power] +=
            term.amplitude * (term.frequency * millennia + term.phase).sin();
    }

    coefficients
        .iter()
        .rev()
        .fold(0.0, |value, coefficient| value * millennia + coefficient)
        / MICROSECONDS_PER_SECOND
}

/// Barycentric Dynamical Time, the uniform time scale used for barycentric
/// ephemerides.
///
/// Conversion to and from TT uses the Fairhead-Bretagnon analytical expansion.
/// The expansion is intended for dates within a few thousand years of J2000.
/// Use [`crate::Time<TDB>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct TDB;

impl Scale for TDB {
    const NAME: &'static str = "TDB";
}

impl_from_anytime!(TDB);

impl From<Time<GPST>> for Time<TDB> {
    fn from(gpst: Time<GPST>) -> Self {
        let tt: Time<TT> = gpst.into();
        tt.into()
    }
}
impl_time_series_from!(GPST => TDB);

impl From<Time<TAI>> for Time<TDB> {
    fn from(tai: Time<TAI>) -> Self {
        let tt: Time<TT> = tai.into();
        tt.into()
    }
}
impl_time_series_from!(TAI => TDB);

impl From<Time<TCG>> for Time<TDB> {
    fn from(tcg: Time<TCG>) -> Self {
        let tt: Time<TT> = tcg.into();
        tt.into()
    }
}
impl_time_series_from!(TCG => TDB);

impl From<Time<TT>> for Time<TDB> {
    fn from(tt: Time<TT>) -> Self {
        let delta_seconds = tdb_minus_tt_seconds(&tt);
        tt.shift_scale_secs(delta_seconds)
    }
}
impl_time_series_from!(TT => TDB);

impl From<Time<UT1>> for Time<TDB> {
    fn from(ut1: Time<UT1>) -> Self {
        let tt: Time<TT> = ut1.into();
        tt.into()
    }
}
impl_time_series_from!(UT1 => TDB);

impl From<Time<UTC>> for Time<TDB> {
    fn from(utc: Time<UTC>) -> Self {
        let tt: Time<TT> = utc.into();
        tt.into()
    }
}
impl_time_series_from!(UTC => TDB);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;
    use chrono::TimeDelta;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn coefficients_have_expected_polynomial_groups() {
        let mut counts = [0; 6];
        for term in terms() {
            counts[term.power] += 1;
        }

        assert_eq!(counts, [971, 457, 207, 45, 16, 1]);
    }

    #[test]
    fn matches_coefficient_checkpoints() {
        let cases = [
            (0.0, -0.000_099_306_404_710),
            (0.1, -0.000_075_619_962_141),
            (-0.1, -0.000_033_320_661_541),
        ];

        for (millennia, expected) in cases {
            let tt = Time::<TT>::from_split_jd(J2000_JD, millennia * DAYS_PER_JULIAN_MILLENNIUM);
            let actual = tdb_minus_tt_seconds(&tt);
            assert!((actual - expected).abs() < 5e-15);
        }
    }

    #[test]
    fn tt_to_tdb_rounds_to_nanoseconds() {
        let tt = Time::<TT>::from_jd(J2000_JD);
        let tdb: Time<TDB> = tt.clone().into();

        assert_eq!(tdb.value - tt.value, TimeDelta::nanoseconds(-99_306));
    }

    #[test]
    fn matches_validation_reference_checkpoints() {
        let cases = [
            (J2000_JD, 0.0, J2000_JD, -1.149_388_876_664_287_8e-9),
            (2_457_754.0, 0.5, 2_457_754.0, 0.499_999_999_426_850_9),
            (
                2_441_685.0,
                0.000_511_388_888_888_910_6,
                2_441_685.0,
                0.000_511_388_593_643_106_5,
            ),
        ];

        for (tt1, tt2, tdb1, tdb2) in cases {
            let actual: Time<TDB> = Time::<TT>::from_split_jd(tt1, tt2).into();
            let expected = Time::<TDB>::from_split_jd(tdb1, tdb2);
            let error = actual.value - expected.value;
            let error_nanoseconds =
                error.num_seconds() * 1_000_000_000 + error.subsec_nanos() as i64;

            assert!(error_nanoseconds.abs() <= 1);
        }
    }

    #[test]
    fn test_tdb_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, TDB>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, TDB>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, TDB>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TT, TDB>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, TDB>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, TDB>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

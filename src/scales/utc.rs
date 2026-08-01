use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCB, TCG, TDB, TT, UT1};
use crate::{Scale, Time};

/// Coordinated Universal Time, the civil time scale with leap seconds.
///
/// UTC days may contain 86,401 seconds when a positive leap second occurs.
/// Use [`crate::Time<UTC>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct UTC;

impl Scale for UTC {
    const NAME: &'static str = "UTC";
    const USES_UTC_DAY_SCALING: bool = true;
}

impl_from_anytime!(UTC);

impl From<Time<GPST>> for Time<UTC> {
    fn from(gpst: Time<GPST>) -> Self {
        let time_tai: Time<TAI> = gpst.into();
        time_tai.into()
    }
}
impl_time_series_from!(GPST => UTC);

impl From<Time<TAI>> for Time<UTC> {
    fn from(tai: Time<TAI>) -> Self {
        let mut utc = tai.shift_scale_secs(0.0);

        for _ in 0..3 {
            let guessed_tai: Time<TAI> = utc.clone().into();
            utc.value += tai.value - guessed_tai.value;
        }

        utc
    }
}
impl_time_series_from!(TAI => UTC);

impl From<Time<TCB>> for Time<UTC> {
    fn from(tcb: Time<TCB>) -> Self {
        let tdb: Time<TDB> = tcb.into();
        tdb.into()
    }
}
impl_time_series_from!(TCB => UTC);

impl From<Time<TCG>> for Time<UTC> {
    fn from(tcg: Time<TCG>) -> Self {
        let time_tai: Time<TAI> = tcg.into();
        time_tai.into()
    }
}
impl_time_series_from!(TCG => UTC);

impl From<Time<TDB>> for Time<UTC> {
    fn from(tdb: Time<TDB>) -> Self {
        let time_tt: Time<TT> = tdb.into();
        time_tt.into()
    }
}
impl_time_series_from!(TDB => UTC);

impl From<Time<TT>> for Time<UTC> {
    fn from(tt: Time<TT>) -> Self {
        let time_tai: Time<TAI> = tt.into();
        time_tai.into()
    }
}
impl_time_series_from!(TT => UTC);

impl From<Time<UT1>> for Time<UTC> {
    fn from(ut1: Time<UT1>) -> Self {
        let mut utc = ut1.shift_scale_secs::<UTC>(0.0);

        for _ in 0..3 {
            let guessed_ut1: Time<UT1> = utc.clone().into();
            utc.value += ut1.value - guessed_ut1.value;
        }

        utc
    }
}
impl_time_series_from!(UT1 => UTC);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;
    use chrono::TimeDelta;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_tai_to_utc_conversion() {
        let tai_time = Time::<TAI>::from_jd(2458000.0);
        let utc_time: Time<UTC> = tai_time.into();
        assert_eq!(utc_time.value, TimeDelta::seconds(2458000 * 86400 - 37));
    }

    #[test]
    fn test_utc_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, UTC>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, UTC>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCB, UTC>(Time::<TCB>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, UTC>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, UTC>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, UTC>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, UTC>(Time::<UT1>::from_jd(2_457_754.5));
    }
}

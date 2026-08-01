use crate::constants::{TAI_GPS, TAI_TT};
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::common::utc_day;
use crate::scales::{GPST, TCB, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// International Atomic Time, the continuous atomic reference time scale.
///
/// Use [`crate::Time<TAI>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct TAI;

impl Scale for TAI {
    const NAME: &'static str = "TAI";
}

impl_from_anytime!(TAI);

impl From<Time<GPST>> for Time<TAI> {
    fn from(gpst: Time<GPST>) -> Self {
        gpst.shift_scale_secs(TAI_GPS)
    }
}
impl_time_series_from!(GPST => TAI);

impl From<Time<TCB>> for Time<TAI> {
    fn from(tcb: Time<TCB>) -> Self {
        let tdb: Time<TDB> = tcb.into();
        tdb.into()
    }
}
impl_time_series_from!(TCB => TAI);

impl From<Time<TCG>> for Time<TAI> {
    fn from(tcg: Time<TCG>) -> Self {
        let time_tt: Time<TT> = tcg.into();
        time_tt.into()
    }
}
impl_time_series_from!(TCG => TAI);

impl From<Time<TDB>> for Time<TAI> {
    fn from(tdb: Time<TDB>) -> Self {
        let time_tt: Time<TT> = tdb.into();
        time_tt.into()
    }
}
impl_time_series_from!(TDB => TAI);

impl From<Time<TT>> for Time<TAI> {
    fn from(tt: Time<TT>) -> Self {
        tt.shift_scale_secs(-TAI_TT)
    }
}
impl_time_series_from!(TT => TAI);

impl From<Time<UT1>> for Time<TAI> {
    fn from(ut1: Time<UT1>) -> Self {
        let utc: Time<UTC> = ut1.into();
        utc.into()
    }
}
impl_time_series_from!(UT1 => TAI);

impl From<Time<UTC>> for Time<TAI> {
    fn from(utc: Time<UTC>) -> Self {
        let (jd1, jd2) = utc.split_jd();
        utc.shift_scale_secs(utc_day(jd1, jd2).tai_minus_utc)
    }
}
impl_time_series_from!(UTC => TAI);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_tai_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, TAI>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TCB, TAI>(Time::<TCB>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, TAI>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, TAI>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, TAI>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, TAI>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, TAI>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

use crate::constants::{DAY_SECONDS, TAI_TT};
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCB, TCG, TDB, UT1, UTC};
use crate::{Scale, Time};

/// Terrestrial Time, the uniform time scale used for geocentric ephemerides.
///
/// TT is ahead of TAI by 32.184 seconds. Use [`crate::Time<TT>`] to associate
/// a value with this scale.
#[derive(Debug, Clone)]
pub struct TT;

impl Scale for TT {
    const NAME: &'static str = "TT";
}

impl_from_anytime!(TT);

impl From<Time<GPST>> for Time<TT> {
    fn from(gpst: Time<GPST>) -> Self {
        let time_tai: Time<TAI> = gpst.into();
        time_tai.into()
    }
}
impl_time_series_from!(GPST => TT);

impl From<Time<TAI>> for Time<TT> {
    fn from(tai: Time<TAI>) -> Self {
        tai.shift_scale_secs(TAI_TT)
    }
}
impl_time_series_from!(TAI => TT);

impl From<Time<TCB>> for Time<TT> {
    fn from(tcb: Time<TCB>) -> Self {
        let tdb: Time<TDB> = tcb.into();
        tdb.into()
    }
}
impl_time_series_from!(TCB => TT);

impl From<Time<TCG>> for Time<TT> {
    fn from(tcg: Time<TCG>) -> Self {
        let delta_secs =
            -6.969290134e-10 * (tcg.mjd() - TAI_TT / DAY_SECONDS - 43144.0) * DAY_SECONDS;
        tcg.shift_scale_secs(delta_secs)
    }
}
impl_time_series_from!(TCG => TT);

impl From<Time<TDB>> for Time<TT> {
    fn from(tdb: Time<TDB>) -> Self {
        let mut tt = tdb.shift_scale_secs::<TT>(0.0);

        let guessed_tdb: Time<TDB> = tt.clone().into();
        tt.value += tdb.value - guessed_tdb.value;

        tt
    }
}
impl_time_series_from!(TDB => TT);

impl From<Time<UT1>> for Time<TT> {
    fn from(ut1: Time<UT1>) -> Self {
        let utc: Time<UTC> = ut1.into();
        utc.into()
    }
}
impl_time_series_from!(UT1 => TT);

impl From<Time<UTC>> for Time<TT> {
    fn from(utc: Time<UTC>) -> Self {
        let time_tai: Time<TAI> = utc.into();
        time_tai.into()
    }
}
impl_time_series_from!(UTC => TT);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_tt_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, TT>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, TT>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCB, TT>(Time::<TCB>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, TT>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, TT>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, TT>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, TT>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

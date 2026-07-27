use crate::constants::{DAY_SECONDS, TAI_TT};
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// Geocentric Coordinate Time, a relativistic coordinate time scale.
///
/// TCG runs at a slightly different rate from TT. Use [`crate::Time<TCG>`]
/// to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct TCG;

impl Scale for TCG {
    const NAME: &'static str = "TCG";
}

impl_from_anytime!(TCG);

impl From<Time<GPST>> for Time<TCG> {
    fn from(gpst: Time<GPST>) -> Self {
        let time_tai: Time<TAI> = gpst.into();
        time_tai.into()
    }
}
impl_time_series_from!(GPST => TCG);

impl From<Time<TAI>> for Time<TCG> {
    fn from(tai: Time<TAI>) -> Self {
        let time_tt: Time<TT> = tai.into();
        time_tt.into()
    }
}
impl_time_series_from!(TAI => TCG);

impl From<Time<TDB>> for Time<TCG> {
    fn from(tdb: Time<TDB>) -> Self {
        let time_tt: Time<TT> = tdb.into();
        time_tt.into()
    }
}
impl_time_series_from!(TDB => TCG);

impl From<Time<TT>> for Time<TCG> {
    fn from(tt: Time<TT>) -> Self {
        let delta_secs = (6.969290134e-10 / (1.0 - 6.969290134e-10))
            * (tt.mjd() - 43144.0 - TAI_TT / DAY_SECONDS)
            * DAY_SECONDS;
        tt.shift_scale_secs(delta_secs)
    }
}
impl_time_series_from!(TT => TCG);

impl From<Time<UT1>> for Time<TCG> {
    fn from(ut1: Time<UT1>) -> Self {
        let utc: Time<UTC> = ut1.into();
        utc.into()
    }
}
impl_time_series_from!(UT1 => TCG);

impl From<Time<UTC>> for Time<TCG> {
    fn from(utc: Time<UTC>) -> Self {
        let time_tai: Time<TAI> = utc.into();
        time_tai.into()
    }
}
impl_time_series_from!(UTC => TCG);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_tcg_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, TCG>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, TCG>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, TCG>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, TCG>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, TCG>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, TCG>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

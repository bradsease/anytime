use crate::constants::TAI_GPS;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{TAI, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// GPS time, a continuous atomic time scale maintained by the GPS system.
///
/// GPST is 19 SI seconds behind TAI. Use [`crate::Time<GPST>`] to associate a
/// value with this scale.
#[derive(Debug, Clone)]
pub struct GPST;

impl Scale for GPST {
    const NAME: &'static str = "GPST";
}

impl_from_anytime!(GPST);

impl From<Time<TAI>> for Time<GPST> {
    fn from(tai: Time<TAI>) -> Self {
        tai.shift_scale_secs(-TAI_GPS)
    }
}
impl_time_series_from!(TAI => GPST);

impl From<Time<TCG>> for Time<GPST> {
    fn from(tcg: Time<TCG>) -> Self {
        let time_tai: Time<TAI> = tcg.into();
        time_tai.into()
    }
}
impl_time_series_from!(TCG => GPST);

impl From<Time<TDB>> for Time<GPST> {
    fn from(tdb: Time<TDB>) -> Self {
        let tt: Time<TT> = tdb.into();
        tt.into()
    }
}
impl_time_series_from!(TDB => GPST);

impl From<Time<TT>> for Time<GPST> {
    fn from(tt: Time<TT>) -> Self {
        let time_tai: Time<TAI> = tt.into();
        time_tai.into()
    }
}
impl_time_series_from!(TT => GPST);

impl From<Time<UT1>> for Time<GPST> {
    fn from(ut1: Time<UT1>) -> Self {
        let utc: Time<UTC> = ut1.into();
        utc.into()
    }
}
impl_time_series_from!(UT1 => GPST);

impl From<Time<UTC>> for Time<GPST> {
    fn from(utc: Time<UTC>) -> Self {
        let time_tai: Time<TAI> = utc.into();
        time_tai.into()
    }
}
impl_time_series_from!(UTC => GPST);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_gpst_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<TAI, GPST>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, GPST>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, GPST>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, GPST>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, GPST>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, GPST>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

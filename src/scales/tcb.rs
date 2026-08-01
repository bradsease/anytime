use crate::constants::DAY_SECONDS;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

pub(super) const L_B: f64 = 1.550_519_768e-8;
pub(super) const T_0: f64 = 2_443_144.500_372_5;
pub(super) const TDB_0_SECONDS: f64 = -6.55e-5;

/// Barycentric Coordinate Time, the relativistic coordinate time scale for the
/// solar-system barycentric reference system.
///
/// Conversion to and from TDB uses the linear transformation defined by IAU
/// Resolution B3 (2006). Use [`crate::Time<TCB>`] to associate a value with
/// this scale.
#[derive(Debug, Clone)]
pub struct TCB;

impl Scale for TCB {
    const NAME: &'static str = "TCB";
}

impl_from_anytime!(TCB);

impl From<Time<GPST>> for Time<TCB> {
    fn from(gpst: Time<GPST>) -> Self {
        let tdb: Time<TDB> = gpst.into();
        tdb.into()
    }
}
impl_time_series_from!(GPST => TCB);

impl From<Time<TAI>> for Time<TCB> {
    fn from(tai: Time<TAI>) -> Self {
        let tdb: Time<TDB> = tai.into();
        tdb.into()
    }
}
impl_time_series_from!(TAI => TCB);

impl From<Time<TCG>> for Time<TCB> {
    fn from(tcg: Time<TCG>) -> Self {
        let tdb: Time<TDB> = tcg.into();
        tdb.into()
    }
}
impl_time_series_from!(TCG => TCB);

impl From<Time<TDB>> for Time<TCB> {
    fn from(tdb: Time<TDB>) -> Self {
        let (jd1, jd2) = tdb.split_jd();
        let seconds_since_t_0 = ((jd1 - T_0) + jd2) * DAY_SECONDS;
        let delta_seconds = (L_B * seconds_since_t_0 - TDB_0_SECONDS) / (1.0 - L_B);
        tdb.shift_scale_secs(delta_seconds)
    }
}
impl_time_series_from!(TDB => TCB);

impl From<Time<TT>> for Time<TCB> {
    fn from(tt: Time<TT>) -> Self {
        let tdb: Time<TDB> = tt.into();
        tdb.into()
    }
}
impl_time_series_from!(TT => TCB);

impl From<Time<UT1>> for Time<TCB> {
    fn from(ut1: Time<UT1>) -> Self {
        let tdb: Time<TDB> = ut1.into();
        tdb.into()
    }
}
impl_time_series_from!(UT1 => TCB);

impl From<Time<UTC>> for Time<TCB> {
    fn from(utc: Time<UTC>) -> Self {
        let tdb: Time<TDB> = utc.into();
        tdb.into()
    }
}
impl_time_series_from!(UTC => TCB);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_tcb_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, TCB>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, TCB>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, TCB>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, TCB>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, TCB>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, TCB>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, TCB>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

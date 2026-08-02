use crate::constants::GLONASST_UTC;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{BDT, GPST, GST, QZZST, TAI, TCB, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// GLONASS Time, defined as UTC plus three hours.
///
/// GLONASST follows UTC leap seconds and is 10,800 seconds ahead of UTC. Use
/// [`crate::Time<GLONASST>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct GLONASST;

impl Scale for GLONASST {
    const NAME: &'static str = "GLONASST";
}

impl_from_anytime!(GLONASST);

impl From<Time<UTC>> for Time<GLONASST> {
    fn from(utc: Time<UTC>) -> Self {
        utc.shift_scale_secs(GLONASST_UTC)
    }
}
impl_time_series_from!(UTC => GLONASST);

impl From<Time<GLONASST>> for Time<UTC> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        glonasst.shift_scale_secs(-GLONASST_UTC)
    }
}
impl_time_series_from!(GLONASST => UTC);

impl From<Time<TAI>> for Time<GLONASST> {
    fn from(tai: Time<TAI>) -> Self {
        let utc: Time<UTC> = tai.into();
        utc.into()
    }
}
impl_time_series_from!(TAI => GLONASST);

impl From<Time<GLONASST>> for Time<TAI> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let utc: Time<UTC> = glonasst.into();
        utc.into()
    }
}
impl_time_series_from!(GLONASST => TAI);

impl From<Time<GPST>> for Time<GLONASST> {
    fn from(gpst: Time<GPST>) -> Self {
        let tai: Time<TAI> = gpst.into();
        tai.into()
    }
}
impl_time_series_from!(GPST => GLONASST);

impl From<Time<GLONASST>> for Time<GPST> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => GPST);

impl From<Time<QZZST>> for Time<GLONASST> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => GLONASST);

impl From<Time<GLONASST>> for Time<QZZST> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => QZZST);

impl From<Time<TCB>> for Time<GLONASST> {
    fn from(tcb: Time<TCB>) -> Self {
        let tai: Time<TAI> = tcb.into();
        tai.into()
    }
}
impl_time_series_from!(TCB => GLONASST);

impl From<Time<GLONASST>> for Time<TCB> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => TCB);

impl From<Time<TCG>> for Time<GLONASST> {
    fn from(tcg: Time<TCG>) -> Self {
        let tai: Time<TAI> = tcg.into();
        tai.into()
    }
}
impl_time_series_from!(TCG => GLONASST);

impl From<Time<GLONASST>> for Time<TCG> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => TCG);

impl From<Time<TDB>> for Time<GLONASST> {
    fn from(tdb: Time<TDB>) -> Self {
        let tai: Time<TAI> = tdb.into();
        tai.into()
    }
}
impl_time_series_from!(TDB => GLONASST);

impl From<Time<GLONASST>> for Time<TDB> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => TDB);

impl From<Time<TT>> for Time<GLONASST> {
    fn from(tt: Time<TT>) -> Self {
        let tai: Time<TAI> = tt.into();
        tai.into()
    }
}
impl_time_series_from!(TT => GLONASST);

impl From<Time<GLONASST>> for Time<TT> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => TT);

impl From<Time<UT1>> for Time<GLONASST> {
    fn from(ut1: Time<UT1>) -> Self {
        let tai: Time<TAI> = ut1.into();
        tai.into()
    }
}
impl_time_series_from!(UT1 => GLONASST);

impl From<Time<GLONASST>> for Time<UT1> {
    fn from(glonasst: Time<GLONASST>) -> Self {
        let tai: Time<TAI> = glonasst.into();
        tai.into()
    }
}
impl_time_series_from!(GLONASST => UT1);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;
    use chrono::{NaiveDate, TimeDelta};

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn applies_utc_offset() {
        let utc = Time::<UTC>::from_gregorian(
            NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        let glonasst: Time<GLONASST> = utc.clone().into();

        assert_eq!(glonasst.value - utc.value, TimeDelta::hours(3));
        assert_eq!(glonasst.tai(), utc.tai());
    }

    #[test]
    fn conversions_round_trip() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<BDT, GLONASST>(Time::<BDT>::from_jd(2_457_754.5));
        assert_round_trip::<GPST, GLONASST>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<GST, GLONASST>(Time::<GST>::from_jd(2_457_754.5));
        assert_round_trip::<QZZST, GLONASST>(Time::<QZZST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, GLONASST>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCB, GLONASST>(Time::<TCB>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, GLONASST>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, GLONASST>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, GLONASST>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UT1, GLONASST>(Time::<UT1>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, GLONASST>(Time::<UTC>::from_jd(2_457_754.5));
    }
}

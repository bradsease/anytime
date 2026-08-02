use crate::constants::TAI_GPS;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{TAI, TCB, TCG, TDB, TT, UT1, UTC};
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

impl From<Time<TCB>> for Time<GPST> {
    fn from(tcb: Time<TCB>) -> Self {
        let tdb: Time<TDB> = tcb.into();
        tdb.into()
    }
}
impl_time_series_from!(TCB => GPST);

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
    use chrono::TimeDelta;

    #[test]
    fn applies_tai_offset() {
        let tai = Time::<TAI>::from_jd(2_458_849.5);
        let gpst: Time<GPST> = tai.clone().into();

        assert_eq!(tai.value - gpst.value, TimeDelta::seconds(19));
        assert_eq!(gpst.tai(), tai);
    }
}

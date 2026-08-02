use crate::constants::TAI_GPS;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCB, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// QZSS Time, synchronized with GPS Time.
///
/// QZZST is 19 SI seconds behind TAI and does not apply leap seconds. Use
/// [`crate::Time<QZZST>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct QZZST;

impl Scale for QZZST {
    const NAME: &'static str = "QZZST";
}

impl_from_anytime!(QZZST);

impl From<Time<TAI>> for Time<QZZST> {
    fn from(tai: Time<TAI>) -> Self {
        tai.shift_scale_secs(-TAI_GPS)
    }
}
impl_time_series_from!(TAI => QZZST);

impl From<Time<QZZST>> for Time<TAI> {
    fn from(qzzst: Time<QZZST>) -> Self {
        qzzst.shift_scale_secs(TAI_GPS)
    }
}
impl_time_series_from!(QZZST => TAI);

impl From<Time<GPST>> for Time<QZZST> {
    fn from(gpst: Time<GPST>) -> Self {
        gpst.shift_scale_secs(0.0)
    }
}
impl_time_series_from!(GPST => QZZST);

impl From<Time<QZZST>> for Time<GPST> {
    fn from(qzzst: Time<QZZST>) -> Self {
        qzzst.shift_scale_secs(0.0)
    }
}
impl_time_series_from!(QZZST => GPST);

impl From<Time<UTC>> for Time<QZZST> {
    fn from(utc: Time<UTC>) -> Self {
        let tai: Time<TAI> = utc.into();
        tai.into()
    }
}
impl_time_series_from!(UTC => QZZST);

impl From<Time<QZZST>> for Time<UTC> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => UTC);

impl From<Time<TCB>> for Time<QZZST> {
    fn from(tcb: Time<TCB>) -> Self {
        let tai: Time<TAI> = tcb.into();
        tai.into()
    }
}
impl_time_series_from!(TCB => QZZST);

impl From<Time<QZZST>> for Time<TCB> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => TCB);

impl From<Time<TCG>> for Time<QZZST> {
    fn from(tcg: Time<TCG>) -> Self {
        let tai: Time<TAI> = tcg.into();
        tai.into()
    }
}
impl_time_series_from!(TCG => QZZST);

impl From<Time<QZZST>> for Time<TCG> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => TCG);

impl From<Time<TDB>> for Time<QZZST> {
    fn from(tdb: Time<TDB>) -> Self {
        let tai: Time<TAI> = tdb.into();
        tai.into()
    }
}
impl_time_series_from!(TDB => QZZST);

impl From<Time<QZZST>> for Time<TDB> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => TDB);

impl From<Time<TT>> for Time<QZZST> {
    fn from(tt: Time<TT>) -> Self {
        let tai: Time<TAI> = tt.into();
        tai.into()
    }
}
impl_time_series_from!(TT => QZZST);

impl From<Time<QZZST>> for Time<TT> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => TT);

impl From<Time<UT1>> for Time<QZZST> {
    fn from(ut1: Time<UT1>) -> Self {
        let tai: Time<TAI> = ut1.into();
        tai.into()
    }
}
impl_time_series_from!(UT1 => QZZST);

impl From<Time<QZZST>> for Time<UT1> {
    fn from(qzzst: Time<QZZST>) -> Self {
        let tai: Time<TAI> = qzzst.into();
        tai.into()
    }
}
impl_time_series_from!(QZZST => UT1);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    #[test]
    fn applies_tai_offset() {
        let tai = Time::<TAI>::from_jd(2_458_849.5);
        let qzzst: Time<QZZST> = tai.clone().into();

        assert_eq!(tai.value - qzzst.value, TimeDelta::seconds(19));
        assert_eq!(qzzst.tai(), tai);
    }
}

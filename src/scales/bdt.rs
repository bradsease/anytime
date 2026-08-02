use crate::constants::TAI_BDT;
use crate::macros::{impl_from_anytime, impl_time_series_from, impl_via_tai};
use crate::scales::{GLONASST, GPST, GST, QZZST, TAI, TCB, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// BeiDou Time, a continuous satellite-navigation time scale.
///
/// BDT began at 2006-01-01T00:00:00 UTC and is 33 SI seconds behind TAI. Use
/// [`crate::Time<BDT>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct BDT;

impl Scale for BDT {
    const NAME: &'static str = "BDT";
}

impl_from_anytime!(BDT);

impl From<Time<TAI>> for Time<BDT> {
    fn from(tai: Time<TAI>) -> Self {
        tai.shift_scale_secs(-TAI_BDT)
    }
}
impl_time_series_from!(TAI => BDT);

impl From<Time<BDT>> for Time<TAI> {
    fn from(bdt: Time<BDT>) -> Self {
        bdt.shift_scale_secs(TAI_BDT)
    }
}
impl_time_series_from!(BDT => TAI);

impl_via_tai!(GLONASST => BDT);
impl_via_tai!(GPST => BDT);
impl_via_tai!(GST => BDT);
impl_via_tai!(QZZST => BDT);
impl_via_tai!(TCB => BDT);
impl_via_tai!(TCG => BDT);
impl_via_tai!(TDB => BDT);
impl_via_tai!(TT => BDT);
impl_via_tai!(UT1 => BDT);
impl_via_tai!(UTC => BDT);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    #[test]
    fn applies_tai_offset() {
        let tai = Time::<TAI>::from_jd(2_458_849.5);
        let bdt: Time<BDT> = tai.clone().into();

        assert_eq!(tai.value - bdt.value, TimeDelta::seconds(33));
        assert_eq!(bdt.tai(), tai);
    }
}

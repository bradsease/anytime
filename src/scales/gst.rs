use crate::constants::TAI_GST;
use crate::macros::{impl_from_anytime, impl_time_series_from, impl_via_tai};
use crate::scales::{GLONASST, GPST, QZZST, TAI, TCB, TCG, TDB, TT, UT1, UTC};
use crate::{Scale, Time};

/// Galileo System Time, a continuous satellite-navigation time scale.
///
/// GST began at 1999-08-22T00:00:00 GST and is 19 SI seconds behind TAI. Use
/// [`crate::Time<GST>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct GST;

impl Scale for GST {
    const NAME: &'static str = "GST";
}

impl_from_anytime!(GST);

impl From<Time<TAI>> for Time<GST> {
    fn from(tai: Time<TAI>) -> Self {
        tai.shift_scale_secs(-TAI_GST)
    }
}
impl_time_series_from!(TAI => GST);

impl From<Time<GST>> for Time<TAI> {
    fn from(gst: Time<GST>) -> Self {
        gst.shift_scale_secs(TAI_GST)
    }
}
impl_time_series_from!(GST => TAI);

impl_via_tai!(GLONASST => GST);
impl_via_tai!(GPST => GST);
impl_via_tai!(QZZST => GST);
impl_via_tai!(TCB => GST);
impl_via_tai!(TCG => GST);
impl_via_tai!(TDB => GST);
impl_via_tai!(TT => GST);
impl_via_tai!(UT1 => GST);
impl_via_tai!(UTC => GST);

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    #[test]
    fn applies_tai_offset() {
        let tai = Time::<TAI>::from_jd(2_458_849.5);
        let gst: Time<GST> = tai.clone().into();

        assert_eq!(tai.value - gst.value, TimeDelta::seconds(19));
        assert_eq!(gst.tai(), tai);
    }
}

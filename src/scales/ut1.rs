use crate::eop::sample_ut1_minus_utc;
use crate::macros::{impl_from_anytime, impl_time_series_from};
use crate::scales::{GPST, TAI, TCB, TCG, TDB, TT, UTC};
use crate::{Scale, Time};

/// Universal Time 1, an Earth-rotation time scale based on observed UT1-UTC
/// values.
///
/// UT1 conversion requires a loaded Earth-orientation dataset. If no usable
/// sample is available, conversion currently preserves the UTC instant. Use
/// [`crate::Time<UT1>`] to associate a value with this scale.
#[derive(Debug, Clone)]
pub struct UT1;

impl Scale for UT1 {
    const NAME: &'static str = "UT1";
}

impl_from_anytime!(UT1);

impl From<Time<GPST>> for Time<UT1> {
    fn from(gpst: Time<GPST>) -> Self {
        let utc: Time<UTC> = gpst.into();
        utc.into()
    }
}
impl_time_series_from!(GPST => UT1);

impl From<Time<TAI>> for Time<UT1> {
    fn from(tai: Time<TAI>) -> Self {
        let utc: Time<UTC> = tai.into();
        utc.into()
    }
}
impl_time_series_from!(TAI => UT1);

impl From<Time<TCB>> for Time<UT1> {
    fn from(tcb: Time<TCB>) -> Self {
        let tdb: Time<TDB> = tcb.into();
        tdb.into()
    }
}
impl_time_series_from!(TCB => UT1);

impl From<Time<TCG>> for Time<UT1> {
    fn from(tcg: Time<TCG>) -> Self {
        let utc: Time<UTC> = tcg.into();
        utc.into()
    }
}
impl_time_series_from!(TCG => UT1);

impl From<Time<TDB>> for Time<UT1> {
    fn from(tdb: Time<TDB>) -> Self {
        let utc: Time<UTC> = tdb.into();
        utc.into()
    }
}
impl_time_series_from!(TDB => UT1);

impl From<Time<TT>> for Time<UT1> {
    fn from(tt: Time<TT>) -> Self {
        let utc: Time<UTC> = tt.into();
        utc.into()
    }
}
impl_time_series_from!(TT => UT1);

impl From<Time<UTC>> for Time<UT1> {
    fn from(utc: Time<UTC>) -> Self {
        let dut1 = match sample_ut1_minus_utc(utc.mjd()) {
            Ok(dut1) => dut1,
            Err(_) => return utc.shift_scale_secs(0.0),
        };
        let tai: Time<TAI> = utc.clone().into();
        let utc_day = Time::<UTC>::from_mjd(utc.mjd().floor());
        let tai_at_utc_day: Time<TAI> = utc_day.clone().into();
        let tai_minus_utc = tai_at_utc_day.value - utc_day.value;
        let tai_minus_utc =
            tai_minus_utc.num_seconds() as f64 + tai_minus_utc.subsec_nanos() as f64 * 1e-9;
        tai.shift_scale_secs(dut1 - tai_minus_utc)
    }
}
impl_time_series_from!(UTC => UT1);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_finals2000a;
    use crate::scales::common::assert_round_trip;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    #[test]
    fn test_ut1_closure() {
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_round_trip::<GPST, UT1>(Time::<GPST>::from_jd(2_457_754.5));
        assert_round_trip::<TAI, UT1>(Time::<TAI>::from_jd(2_457_754.5));
        assert_round_trip::<TCB, UT1>(Time::<TCB>::from_jd(2_457_754.5));
        assert_round_trip::<TCG, UT1>(Time::<TCG>::from_jd(2_457_754.5));
        assert_round_trip::<TDB, UT1>(Time::<TDB>::from_jd(2_457_754.5));
        assert_round_trip::<TT, UT1>(Time::<TT>::from_jd(2_457_754.5));
        assert_round_trip::<UTC, UT1>(Time::<UTC>::from_jd(2_457_754.5));
    }

    #[test]
    fn assumes_zero_offset_outside_loaded_eop_range() {
        let utc = Time::<UTC>::from_mjd(70_000.0);
        let ut1: Time<UT1> = utc.clone().into();
        let round_trip: Time<UTC> = ut1.into();

        assert_eq!(utc.value, round_trip.value);
    }
}

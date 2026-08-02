use crate::{scales, Time, TimeScale};
use chrono::{NaiveDateTime, ParseError, TimeDelta};
use std::cmp::Ordering;
use std::ops::Sub;

/// A time value whose scale is selected at runtime.
///
/// `AnyTime` is useful for heterogeneous collections. Values are compared and
/// subtracted as physical instants, so the two operands do not need to use the
/// same scale.
///
/// # Examples
///
/// ```
/// use anytime::{AnyTime, Time, scales::{TAI, UTC}};
/// use chrono::TimeDelta;
///
/// let utc = AnyTime::UTC(Time::<UTC>::from_jd(2_451_545.0));
/// let tai = AnyTime::TAI(utc.clone().tai());
/// assert_eq!(tai - utc, TimeDelta::zero());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnyTime {
    /// A value on the BeiDou time scale.
    BDT(Time<scales::BDT>),
    /// A value on the GLONASS time scale.
    GLONASST(Time<scales::GLONASST>),
    /// A value on the GPS time scale.
    GPST(Time<scales::GPST>),
    /// A value on the Galileo System Time scale.
    GST(Time<scales::GST>),
    /// A value on the QZSS time scale.
    QZZST(Time<scales::QZZST>),
    /// A value on the International Atomic Time scale.
    TAI(Time<scales::TAI>),
    /// A value on the Barycentric Coordinate Time scale.
    TCB(Time<scales::TCB>),
    /// A value on the Geocentric Coordinate Time scale.
    TCG(Time<scales::TCG>),
    /// A value on the Barycentric Dynamical Time scale.
    TDB(Time<scales::TDB>),
    /// A value on the Terrestrial Time scale.
    TT(Time<scales::TT>),
    /// A value on the Universal Time 1 scale.
    UT1(Time<scales::UT1>),
    /// A value on the Coordinated Universal Time scale.
    UTC(Time<scales::UTC>),
}

impl AnyTime {
    /// Returns the scale associated with this time.
    pub const fn scale(&self) -> TimeScale {
        match self {
            Self::BDT(_) => TimeScale::BDT,
            Self::GLONASST(_) => TimeScale::GLONASST,
            Self::GPST(_) => TimeScale::GPST,
            Self::GST(_) => TimeScale::GST,
            Self::QZZST(_) => TimeScale::QZZST,
            Self::TAI(_) => TimeScale::TAI,
            Self::TCB(_) => TimeScale::TCB,
            Self::TCG(_) => TimeScale::TCG,
            Self::TDB(_) => TimeScale::TDB,
            Self::TT(_) => TimeScale::TT,
            Self::UT1(_) => TimeScale::UT1,
            Self::UTC(_) => TimeScale::UTC,
        }
    }

    /// Creates a time from a proleptic Gregorian date and time in `scale`.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{AnyTime, TimeScale};
    /// use chrono::NaiveDate;
    ///
    /// let datetime = NaiveDate::from_ymd_opt(2000, 1, 1)
    ///     .unwrap()
    ///     .and_hms_opt(12, 0, 0)
    ///     .unwrap();
    /// let time = AnyTime::from_datetime(datetime, TimeScale::UTC);
    /// assert_eq!(time, AnyTime::from_jd(2_451_545.0, TimeScale::UTC));
    /// ```
    pub fn from_datetime(datetime: NaiveDateTime, scale: TimeScale) -> Self {
        match scale {
            TimeScale::BDT => Time::<scales::BDT>::from_gregorian(datetime).into(),
            TimeScale::GLONASST => Time::<scales::GLONASST>::from_gregorian(datetime).into(),
            TimeScale::GPST => Time::<scales::GPST>::from_gregorian(datetime).into(),
            TimeScale::GST => Time::<scales::GST>::from_gregorian(datetime).into(),
            TimeScale::QZZST => Time::<scales::QZZST>::from_gregorian(datetime).into(),
            TimeScale::TAI => Time::<scales::TAI>::from_gregorian(datetime).into(),
            TimeScale::TCB => Time::<scales::TCB>::from_gregorian(datetime).into(),
            TimeScale::TCG => Time::<scales::TCG>::from_gregorian(datetime).into(),
            TimeScale::TDB => Time::<scales::TDB>::from_gregorian(datetime).into(),
            TimeScale::TT => Time::<scales::TT>::from_gregorian(datetime).into(),
            TimeScale::UT1 => Time::<scales::UT1>::from_gregorian(datetime).into(),
            TimeScale::UTC => Time::<scales::UTC>::from_gregorian(datetime).into(),
        }
    }

    /// Creates a time from a Julian Date in `scale`.
    pub fn from_jd(jd: f64, scale: TimeScale) -> Self {
        match scale {
            TimeScale::BDT => Time::<scales::BDT>::from_jd(jd).into(),
            TimeScale::GLONASST => Time::<scales::GLONASST>::from_jd(jd).into(),
            TimeScale::GPST => Time::<scales::GPST>::from_jd(jd).into(),
            TimeScale::GST => Time::<scales::GST>::from_jd(jd).into(),
            TimeScale::QZZST => Time::<scales::QZZST>::from_jd(jd).into(),
            TimeScale::TAI => Time::<scales::TAI>::from_jd(jd).into(),
            TimeScale::TCB => Time::<scales::TCB>::from_jd(jd).into(),
            TimeScale::TCG => Time::<scales::TCG>::from_jd(jd).into(),
            TimeScale::TDB => Time::<scales::TDB>::from_jd(jd).into(),
            TimeScale::TT => Time::<scales::TT>::from_jd(jd).into(),
            TimeScale::UT1 => Time::<scales::UT1>::from_jd(jd).into(),
            TimeScale::UTC => Time::<scales::UTC>::from_jd(jd).into(),
        }
    }

    /// Creates a time from a Modified Julian Date in `scale`.
    pub fn from_mjd(mjd: f64, scale: TimeScale) -> Self {
        match scale {
            TimeScale::BDT => Time::<scales::BDT>::from_mjd(mjd).into(),
            TimeScale::GLONASST => Time::<scales::GLONASST>::from_mjd(mjd).into(),
            TimeScale::GPST => Time::<scales::GPST>::from_mjd(mjd).into(),
            TimeScale::GST => Time::<scales::GST>::from_mjd(mjd).into(),
            TimeScale::QZZST => Time::<scales::QZZST>::from_mjd(mjd).into(),
            TimeScale::TAI => Time::<scales::TAI>::from_mjd(mjd).into(),
            TimeScale::TCB => Time::<scales::TCB>::from_mjd(mjd).into(),
            TimeScale::TCG => Time::<scales::TCG>::from_mjd(mjd).into(),
            TimeScale::TDB => Time::<scales::TDB>::from_mjd(mjd).into(),
            TimeScale::TT => Time::<scales::TT>::from_mjd(mjd).into(),
            TimeScale::UT1 => Time::<scales::UT1>::from_mjd(mjd).into(),
            TimeScale::UTC => Time::<scales::UTC>::from_mjd(mjd).into(),
        }
    }

    /// Creates a time from a two-part Julian Date in `scale`.
    pub fn from_split_jd(jd1: f64, jd2: f64, scale: TimeScale) -> Self {
        match scale {
            TimeScale::BDT => Time::<scales::BDT>::from_split_jd(jd1, jd2).into(),
            TimeScale::GLONASST => Time::<scales::GLONASST>::from_split_jd(jd1, jd2).into(),
            TimeScale::GPST => Time::<scales::GPST>::from_split_jd(jd1, jd2).into(),
            TimeScale::GST => Time::<scales::GST>::from_split_jd(jd1, jd2).into(),
            TimeScale::QZZST => Time::<scales::QZZST>::from_split_jd(jd1, jd2).into(),
            TimeScale::TAI => Time::<scales::TAI>::from_split_jd(jd1, jd2).into(),
            TimeScale::TCB => Time::<scales::TCB>::from_split_jd(jd1, jd2).into(),
            TimeScale::TCG => Time::<scales::TCG>::from_split_jd(jd1, jd2).into(),
            TimeScale::TDB => Time::<scales::TDB>::from_split_jd(jd1, jd2).into(),
            TimeScale::TT => Time::<scales::TT>::from_split_jd(jd1, jd2).into(),
            TimeScale::UT1 => Time::<scales::UT1>::from_split_jd(jd1, jd2).into(),
            TimeScale::UTC => Time::<scales::UTC>::from_split_jd(jd1, jd2).into(),
        }
    }

    /// Parses an ISO 8601 `T`-separated date and time without an offset in `scale`.
    ///
    /// The input must use the `YYYY-MM-DDTHH:MM:SS` form and may include a
    /// fractional-second component with up to nanosecond precision.
    pub fn from_isot_str(isot: &str, scale: TimeScale) -> Result<Self, ParseError> {
        Self::from_str(isot, "%Y-%m-%dT%H:%M:%S%.f", scale)
    }

    /// Parses an ISO space-separated date and time without an offset in `scale`.
    ///
    /// The input must use the `YYYY-MM-DD HH:MM:SS` form and may include a
    /// fractional-second component with up to nanosecond precision.
    pub fn from_iso_str(iso: &str, scale: TimeScale) -> Result<Self, ParseError> {
        Self::from_str(iso, "%Y-%m-%d %H:%M:%S%.f", scale)
    }

    /// Parses a Chrono-formatted naive date and time string in `scale`.
    ///
    /// `format` accepts the full set of Chrono strftime specifiers.
    pub fn from_str(input: &str, format: &str, scale: TimeScale) -> Result<Self, ParseError> {
        NaiveDateTime::parse_from_str(input, format)
            .map(|datetime| Self::from_datetime(datetime, scale))
    }
}

impl From<Time<scales::GPST>> for AnyTime {
    fn from(time: Time<scales::GPST>) -> Self {
        Self::GPST(time)
    }
}

impl From<Time<scales::BDT>> for AnyTime {
    fn from(time: Time<scales::BDT>) -> Self {
        Self::BDT(time)
    }
}

impl From<Time<scales::GLONASST>> for AnyTime {
    fn from(time: Time<scales::GLONASST>) -> Self {
        Self::GLONASST(time)
    }
}

impl From<Time<scales::GST>> for AnyTime {
    fn from(time: Time<scales::GST>) -> Self {
        Self::GST(time)
    }
}

impl From<Time<scales::QZZST>> for AnyTime {
    fn from(time: Time<scales::QZZST>) -> Self {
        Self::QZZST(time)
    }
}

impl From<Time<scales::TAI>> for AnyTime {
    fn from(time: Time<scales::TAI>) -> Self {
        Self::TAI(time)
    }
}

impl From<Time<scales::TCB>> for AnyTime {
    fn from(time: Time<scales::TCB>) -> Self {
        Self::TCB(time)
    }
}

impl From<Time<scales::TCG>> for AnyTime {
    fn from(time: Time<scales::TCG>) -> Self {
        Self::TCG(time)
    }
}

impl From<Time<scales::TDB>> for AnyTime {
    fn from(time: Time<scales::TDB>) -> Self {
        Self::TDB(time)
    }
}

impl From<Time<scales::TT>> for AnyTime {
    fn from(time: Time<scales::TT>) -> Self {
        Self::TT(time)
    }
}

impl From<Time<scales::UT1>> for AnyTime {
    fn from(time: Time<scales::UT1>) -> Self {
        Self::UT1(time)
    }
}

impl From<Time<scales::UTC>> for AnyTime {
    fn from(time: Time<scales::UTC>) -> Self {
        Self::UTC(time)
    }
}

impl Sub for AnyTime {
    type Output = TimeDelta;

    fn sub(self, other: AnyTime) -> TimeDelta {
        let t1: Time<scales::TAI> = self.into();
        let t2: Time<scales::TAI> = other.into();
        t1 - t2
    }
}

impl PartialEq for AnyTime {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for AnyTime {}

impl PartialOrd for AnyTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnyTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::BDT(lhs) => lhs.cmp(&other.clone().bdt()),
            Self::GLONASST(lhs) => lhs.cmp(&other.clone().glonasst()),
            Self::GPST(lhs) => lhs.cmp(&other.clone().gpst()),
            Self::GST(lhs) => lhs.cmp(&other.clone().gst()),
            Self::QZZST(lhs) => lhs.cmp(&other.clone().qzzst()),
            Self::TAI(lhs) => lhs.cmp(&other.clone().tai()),
            Self::TCB(lhs) => lhs.cmp(&other.clone().tcb()),
            Self::TCG(lhs) => lhs.cmp(&other.clone().tcg()),
            Self::TDB(lhs) => lhs.cmp(&other.clone().tdb()),
            Self::TT(lhs) => lhs.cmp(&other.clone().tt()),
            Self::UT1(lhs) => lhs.cmp(&other.clone().ut1()),
            Self::UTC(lhs) => lhs.cmp(&other.clone().utc()),
        }
    }
}

/// A growable collection of time values that may use different scales.
pub type AnyTimeVec = Vec<AnyTime>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::cmp::Ordering;

    #[test]
    fn test_into() {
        let time_tai = Time::<scales::TAI>::from_jd(2400000.5);
        let any_time = AnyTime::TAI(time_tai);
        let _: Time<scales::UTC> = any_time.clone().into();

        let variants = [
            AnyTime::BDT(Time::<scales::BDT>::from_jd(2400000.5)),
            AnyTime::GLONASST(Time::<scales::GLONASST>::from_jd(2400000.5)),
            AnyTime::GPST(Time::<scales::GPST>::from_jd(2400000.5)),
            AnyTime::GST(Time::<scales::GST>::from_jd(2400000.5)),
            AnyTime::QZZST(Time::<scales::QZZST>::from_jd(2400000.5)),
            any_time,
            AnyTime::TCB(Time::<scales::TCB>::from_jd(2400000.5)),
            AnyTime::TCG(Time::<scales::TCG>::from_jd(2400000.5)),
            AnyTime::TDB(Time::<scales::TDB>::from_jd(2400000.5)),
            AnyTime::TT(Time::<scales::TT>::from_jd(2400000.5)),
            AnyTime::UT1(Time::<scales::UT1>::from_jd(2400000.5)),
            AnyTime::UTC(Time::<scales::UTC>::from_jd(2400000.5)),
        ];

        for variant in variants {
            let _: Time<scales::TAI> = variant.into();
        }
    }

    #[test]
    fn test_constructors() {
        let datetime = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, 123_456_789)
            .unwrap();

        assert_eq!(
            AnyTime::from_datetime(datetime, TimeScale::UTC),
            AnyTime::UTC(Time::<scales::UTC>::from_gregorian(datetime))
        );
        assert_eq!(
            AnyTime::from_jd(2_451_545.0, TimeScale::TAI),
            AnyTime::TAI(Time::<scales::TAI>::from_jd(2_451_545.0))
        );
        assert_eq!(
            AnyTime::from_mjd(51_544.5, TimeScale::TT),
            AnyTime::TT(Time::<scales::TT>::from_mjd(51_544.5))
        );
        assert_eq!(
            AnyTime::from_split_jd(2_451_545.0, 0.25, TimeScale::GPST),
            AnyTime::GPST(Time::<scales::GPST>::from_split_jd(2_451_545.0, 0.25))
        );
        assert!(matches!(
            AnyTime::from_jd(2_451_545.0, TimeScale::TCB),
            AnyTime::TCB(_)
        ));
        assert!(matches!(
            AnyTime::from_jd(2_451_545.0, TimeScale::TCG),
            AnyTime::TCG(_)
        ));
        assert!(matches!(
            AnyTime::from_jd(2_451_545.0, TimeScale::TDB),
            AnyTime::TDB(_)
        ));
        assert!(matches!(
            AnyTime::from_jd(2_451_545.0, TimeScale::UT1),
            AnyTime::UT1(_)
        ));
        assert_eq!(
            AnyTime::from_isot_str("2000-01-01T12:00:00.123456789", TimeScale::UTC).unwrap(),
            AnyTime::from_datetime(datetime, TimeScale::UTC)
        );
        assert_eq!(
            AnyTime::from_iso_str("2000-01-01 12:00:00.123456789", TimeScale::UTC).unwrap(),
            AnyTime::from_datetime(datetime, TimeScale::UTC)
        );
        assert_eq!(
            AnyTime::from_str(
                "2000-01-01 12:00:00.123456789",
                "%Y-%m-%d %H:%M:%S%.f",
                TimeScale::UTC,
            )
            .unwrap(),
            AnyTime::from_datetime(datetime, TimeScale::UTC)
        );
        assert_eq!(
            AnyTime::from_str("00/01/01 12:00", "%y/%m/%d %H:%M", TimeScale::UTC).unwrap(),
            AnyTime::from_datetime(
                NaiveDate::from_ymd_opt(2000, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                TimeScale::UTC,
            )
        );
        assert!(AnyTime::from_isot_str("not a datetime", TimeScale::UTC).is_err());
    }

    #[test]
    fn test_scale() {
        let times = [
            AnyTime::from_jd(2_451_545.0, TimeScale::BDT),
            AnyTime::from_jd(2_451_545.0, TimeScale::GLONASST),
            AnyTime::from_jd(2_451_545.0, TimeScale::GPST),
            AnyTime::from_jd(2_451_545.0, TimeScale::GST),
            AnyTime::from_jd(2_451_545.0, TimeScale::QZZST),
            AnyTime::from_jd(2_451_545.0, TimeScale::TAI),
            AnyTime::from_jd(2_451_545.0, TimeScale::TCB),
            AnyTime::from_jd(2_451_545.0, TimeScale::TCG),
            AnyTime::from_jd(2_451_545.0, TimeScale::TDB),
            AnyTime::from_jd(2_451_545.0, TimeScale::TT),
            AnyTime::from_jd(2_451_545.0, TimeScale::UT1),
            AnyTime::from_jd(2_451_545.0, TimeScale::UTC),
        ];
        let scales = [
            TimeScale::BDT,
            TimeScale::GLONASST,
            TimeScale::GPST,
            TimeScale::GST,
            TimeScale::QZZST,
            TimeScale::TAI,
            TimeScale::TCB,
            TimeScale::TCG,
            TimeScale::TDB,
            TimeScale::TT,
            TimeScale::UT1,
            TimeScale::UTC,
        ];

        for (time, scale) in times.iter().zip(scales) {
            assert_eq!(time.scale(), scale);
        }
    }

    #[test]
    fn test_subtraction() {
        let tai = Time::<scales::TAI>::new(TimeDelta::seconds(100));
        let utc: Time<scales::UTC> = tai.clone().into();

        assert_eq!(AnyTime::TAI(tai) - AnyTime::UTC(utc), TimeDelta::zero());
    }

    #[test]
    fn test_comparison() {
        let earlier_time = Time::<scales::TAI>::new(TimeDelta::seconds(100));
        let same_time: Time<scales::TT> = earlier_time.clone().into();
        let later_time = Time::<scales::TAI>::new(TimeDelta::seconds(101));

        let earlier = AnyTime::TAI(earlier_time);
        let same = AnyTime::TT(same_time);
        let later = AnyTime::TAI(later_time);

        assert_eq!(earlier, same);
        assert_eq!(same, earlier);
        assert!(earlier < later);
        assert!(same < later);
        assert!(later > same);
        assert_eq!(earlier.cmp(&same), Ordering::Equal);
        assert_eq!(same.cmp(&earlier), Ordering::Equal);
        assert_eq!(earlier.cmp(&later), Ordering::Less);
        assert_eq!(later.cmp(&same), Ordering::Greater);

        let variants = [
            AnyTime::BDT(Time::<scales::BDT>::new(TimeDelta::seconds(100))),
            AnyTime::GLONASST(Time::<scales::GLONASST>::new(TimeDelta::seconds(100))),
            AnyTime::GPST(Time::<scales::GPST>::new(TimeDelta::seconds(100))),
            AnyTime::GST(Time::<scales::GST>::new(TimeDelta::seconds(100))),
            AnyTime::QZZST(Time::<scales::QZZST>::new(TimeDelta::seconds(100))),
            AnyTime::TAI(Time::<scales::TAI>::new(TimeDelta::seconds(100))),
            AnyTime::TCB(Time::<scales::TCB>::new(TimeDelta::seconds(100))),
            AnyTime::TCG(Time::<scales::TCG>::new(TimeDelta::seconds(100))),
            AnyTime::TDB(Time::<scales::TDB>::new(TimeDelta::seconds(100))),
            AnyTime::TT(Time::<scales::TT>::new(TimeDelta::seconds(100))),
            AnyTime::UT1(Time::<scales::UT1>::new(TimeDelta::seconds(100))),
            AnyTime::UTC(Time::<scales::UTC>::new(TimeDelta::seconds(100))),
        ];

        for variant in variants {
            assert_eq!(variant.cmp(&variant), Ordering::Equal);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let time = AnyTime::from_jd(2_451_545.0, TimeScale::UTC);
        let json = serde_json::to_string(&time).unwrap();

        assert_eq!(serde_json::from_str::<AnyTime>(&json).unwrap(), time);
    }
}

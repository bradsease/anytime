use crate::{scales, Time};
use chrono::TimeDelta;
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
pub enum AnyTime {
    /// A value on the GPS time scale.
    GPST(Time<scales::GPST>),
    /// A value on the International Atomic Time scale.
    TAI(Time<scales::TAI>),
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
            Self::GPST(lhs) => lhs.cmp(&other.clone().gpst()),
            Self::TAI(lhs) => lhs.cmp(&other.clone().tai()),
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
    use std::cmp::Ordering;

    #[test]
    fn test_into() {
        let time_tai = Time::<scales::TAI>::from_jd(2400000.5);
        let any_time = AnyTime::TAI(time_tai);
        let _: Time<scales::UTC> = any_time.clone().into();

        let variants = [
            AnyTime::GPST(Time::<scales::GPST>::from_jd(2400000.5)),
            any_time,
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
            AnyTime::GPST(Time::<scales::GPST>::new(TimeDelta::seconds(100))),
            AnyTime::TAI(Time::<scales::TAI>::new(TimeDelta::seconds(100))),
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
}

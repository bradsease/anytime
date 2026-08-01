use crate::anytime::AnyTime;
use crate::constants::{DAY_SECONDS, JD_TO_MJD, JD_TO_UNIX_SECONDS};
use crate::macros::impl_to_scale;
use crate::scales;
use chrono::{NaiveDate, NaiveDateTime, TimeDelta, Timelike};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Sub;

/// A time value associated with a specific time scale.
///
/// The scale is represented by the type parameter `S`, so conversions between
/// scales are checked by the compiler. The value has nanosecond precision and
/// can be represented as a Julian Date, Modified Julian Date, or Gregorian
/// date and time.
///
/// # Examples
///
/// ```
/// use anytime::{Time, scales::UTC};
/// use chrono::NaiveDate;
///
/// let time = Time::<UTC>::from_gregorian(
///     NaiveDate::from_ymd_opt(2000, 1, 1)
///         .unwrap()
///         .and_hms_opt(12, 0, 0)
///         .unwrap(),
/// );
/// assert_eq!(time.jd(), 2_451_545.0);
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time<S> {
    /// Duration since Julian date epoch.
    pub(crate) value: TimeDelta,

    /// Marker for the time scale.
    _scale: PhantomData<S>,
}

impl<S> Clone for Time<S> {
    fn clone(&self) -> Self {
        Self {
            value: self.value,
            _scale: PhantomData,
        }
    }
}

/// Describes a time scale that can parameterize [`Time`].
///
/// The crate implements this trait for the marker types in [`crate::scales`].
/// It is public so generic code can refer to a time scale by its associated
/// name and properties.
pub trait Scale {
    /// The standard acronym for the scale.
    const NAME: &'static str;

    /// Whether the scale uses UTC's variable-length civil day.
    ///
    /// This is `true` only for [`crate::scales::UTC`]. Other supported scales
    /// use uniform 86,400-second days.
    const USES_UTC_DAY_SCALING: bool = false;
}

impl<S: Scale> Time<S> {
    pub(crate) fn new(value: TimeDelta) -> Self {
        Time {
            value,
            _scale: PhantomData,
        }
    }

    /// Returns the Julian Date representation of this time.
    ///
    /// The result is the sum of [`Self::split_jd`]'s two components. Use the
    /// split representation when preserving floating-point precision matters.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{Time, scales::TAI};
    ///
    /// let time = Time::<TAI>::from_jd(2_451_545.0);
    /// assert_eq!(time.jd(), 2_451_545.0);
    /// ```
    pub fn jd(&self) -> f64 {
        let seconds = self.value.num_seconds() as f64;
        let nanoseconds = self.value.subsec_nanos() as f64;
        (seconds + nanoseconds * 1e-9) / DAY_SECONDS
    }

    /// Returns the Julian Date as two components for improved precision.
    ///
    /// The returned pair `(jd1, jd2)` represents `jd1 + jd2`.
    pub fn split_jd(&self) -> (f64, f64) {
        let seconds = self.value.num_seconds();
        let nanoseconds = self.value.subsec_nanos();
        let jd1 = (seconds / 86400) as f64;
        let jd2 = ((seconds % 86400) as f64 + nanoseconds as f64 * 1e-9) / DAY_SECONDS;
        (jd1, jd2)
    }

    /// Returns the Modified Julian Date representation of this time.
    ///
    /// Modified Julian Date is Julian Date minus 2,400,000.5.
    pub fn mjd(&self) -> f64 {
        self.jd() - JD_TO_MJD
    }

    /// Creates a time from a Julian Date.
    ///
    /// The fractional part of the date is converted to nanoseconds and may
    /// therefore be rounded to the nearest nanosecond.
    pub fn from_jd(jd: f64) -> Self {
        let total_seconds = jd * DAY_SECONDS;
        let seconds = total_seconds.trunc() as i64;
        let nanoseconds = ((total_seconds.fract()) * 1e9) as u32;
        Time::new(TimeDelta::new(seconds, nanoseconds).unwrap())
    }

    /// Creates a time from a two-part Julian Date.
    ///
    /// Splitting a date into a large integral part and a small fractional part
    /// reduces loss of precision in calculations involving distant dates.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{Time, scales::UTC};
    ///
    /// let time = Time::<UTC>::from_split_jd(2_451_545.0, 0.0);
    /// assert_eq!(time.gregorian().to_string(), "2000-01-01 12:00:00");
    /// ```
    pub fn from_split_jd(jd1: f64, jd2: f64) -> Self {
        let jd1_seconds = (jd1 * DAY_SECONDS).round() as i64;
        let jd2_seconds = jd2 * DAY_SECONDS;
        let seconds = jd1_seconds + jd2_seconds.trunc() as i64;
        let nanoseconds = (jd2_seconds.fract() * 1e9).round() as i64;

        Time::new(TimeDelta::seconds(seconds) + TimeDelta::nanoseconds(nanoseconds))
    }

    /// Creates a time from a Modified Julian Date.
    pub fn from_mjd(mjd: f64) -> Self {
        Self::from_jd(mjd + JD_TO_MJD)
    }

    /// Returns this time as a proleptic Gregorian date and time in its scale.
    ///
    /// For UTC, a leap second is represented using Chrono's leap-second form,
    /// where the nanosecond component can reach one billion.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{Time, scales::UTC};
    ///
    /// let time = Time::<UTC>::from_jd(2_451_545.0);
    /// assert_eq!(time.gregorian().to_string(), "2000-01-01 12:00:00");
    /// ```
    pub fn gregorian(&self) -> NaiveDateTime {
        if !S::USES_UTC_DAY_SCALING {
            return uniform_gregorian(self.value);
        }

        let (jd1, jd2) = self.split_jd();
        let day = scales::common::utc_day(jd1, jd2);
        let midnight = Self::from_jd(day.midnight_jd).value;
        let nanoseconds = (day.scaled_fractional_day * DAY_SECONDS * 1e9).round() as i64;

        if nanoseconds < 86_400_000_000_000 {
            uniform_gregorian(midnight + TimeDelta::nanoseconds(nanoseconds))
        } else {
            let date = uniform_gregorian(midnight).date();
            date.and_hms_nano_opt(23, 59, 59, (nanoseconds - 86_399_000_000_000) as u32)
                .expect("valid Gregorian leap-second time")
        }
    }

    /// Creates a time from a proleptic Gregorian date and time in its scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{Time, scales::UTC};
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2000, 1, 1)
    ///     .unwrap()
    ///     .and_hms_opt(12, 0, 0)
    ///     .unwrap();
    /// let time = Time::<UTC>::from_gregorian(date);
    /// assert_eq!(time.gregorian(), date);
    /// ```
    pub fn from_gregorian(gregorian: NaiveDateTime) -> Self {
        if !S::USES_UTC_DAY_SCALING {
            return Self::new(uniform_time(gregorian));
        }

        let midnight = gregorian
            .date()
            .and_hms_opt(0, 0, 0)
            .expect("valid Gregorian midnight");
        let midnight_value = uniform_time(midnight);
        let midnight_time = Self::new(midnight_value);
        let (jd1, jd2) = midnight_time.split_jd();
        let day = scales::common::utc_day(jd1, jd2);
        let nanoseconds = gregorian.time().num_seconds_from_midnight() as i64 * 1_000_000_000
            + gregorian.time().nanosecond() as i64;

        Self::new(
            midnight_value
                + TimeDelta::nanoseconds(
                    (nanoseconds as f64 / day.fractional_day_scale).round() as i64
                ),
        )
    }

    pub(crate) fn shift_scale_secs<T: Scale>(&self, seconds: f64) -> Time<T> {
        let nanoseconds = (seconds * 1e9).round() as i64;
        Time::<T>::new(self.value + TimeDelta::nanoseconds(nanoseconds))
    }
}

fn unix_epoch() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(1970, 1, 1)
        .expect("valid Unix epoch date")
        .and_hms_opt(0, 0, 0)
        .expect("valid Unix epoch time")
}

fn uniform_gregorian(value: TimeDelta) -> NaiveDateTime {
    unix_epoch() + value - TimeDelta::seconds(JD_TO_UNIX_SECONDS)
}

fn uniform_time(gregorian: NaiveDateTime) -> TimeDelta {
    gregorian - unix_epoch() + TimeDelta::seconds(JD_TO_UNIX_SECONDS)
}

impl<S, T> Sub<Time<T>> for Time<S>
where
    Time<S>: Into<Time<scales::TAI>>,
    Time<T>: Into<Time<scales::TAI>>,
{
    type Output = TimeDelta;

    fn sub(self, other: Time<T>) -> TimeDelta {
        let lhs: Time<scales::TAI> = self.into();
        let rhs: Time<scales::TAI> = other.into();
        lhs.value - rhs.value
    }
}

impl<S, T> PartialEq<Time<T>> for Time<S>
where
    Time<T>: Into<Time<S>>,
{
    fn eq(&self, other: &Time<T>) -> bool {
        let other: Time<S> = (*other).clone().into();
        self.value == other.value
    }
}

impl<S> Eq for Time<S> {}

impl<S, T> PartialOrd<Time<T>> for Time<S>
where
    Time<T>: Into<Time<S>>,
{
    fn partial_cmp(&self, other: &Time<T>) -> Option<Ordering> {
        let other: Time<S> = (*other).clone().into();
        Some(self.value.cmp(&other.value))
    }
}

impl<S> Ord for Time<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl_to_scale!(scales::GPST, gpst);
impl_to_scale!(scales::TAI, tai);
impl_to_scale!(scales::TCB, tcb);
impl_to_scale!(scales::TCG, tcg);
impl_to_scale!(scales::TDB, tdb);
impl_to_scale!(scales::TT, tt);
impl_to_scale!(scales::UT1, ut1);
impl_to_scale!(scales::UTC, utc);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scales::{TAI, TT, UTC};

    #[test]
    fn test_subtraction() {
        let tai1 = Time::<TAI>::new(TimeDelta::seconds(100));
        let tai2 = Time::<TAI>::new(TimeDelta::seconds(90));
        assert_eq!(tai1 - tai2, TimeDelta::seconds(10));

        let tai1 = Time::<TAI>::new(TimeDelta::seconds(90));
        let tai2 = Time::<TAI>::new(TimeDelta::seconds(100));
        assert_eq!(tai1 - tai2, TimeDelta::seconds(-10));

        let start = Time::<UTC>::from_gregorian(
            NaiveDate::from_ymd_opt(2016, 12, 31)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        let end = Time::<UTC>::from_gregorian(
            NaiveDate::from_ymd_opt(2017, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        assert_eq!(end - start, TimeDelta::seconds(86_401));
    }

    #[test]
    fn test_comparison() {
        let earlier = Time::<TAI>::new(TimeDelta::seconds(100));
        let same: Time<TT> = earlier.clone().into();
        let later = Time::<TAI>::new(TimeDelta::seconds(101));

        assert_eq!(earlier, same);
        assert_ne!(earlier, later);
        assert!(earlier < later);
        assert!(earlier <= same);
        assert!(later > same);
        assert!(later >= same);

        let later_tt: Time<TT> = later.clone().into();
        assert_eq!(same, earlier);
        assert_ne!(later_tt, earlier);
        assert!(same < later);
        assert!(same <= earlier);
        assert!(later_tt > earlier);
        assert!(later_tt >= earlier);
    }

    #[test]
    fn test_comparison_across_utc_leap_second() {
        let leap_second = Time::<UTC>::from_gregorian(
            NaiveDate::from_ymd_opt(2016, 12, 31)
                .unwrap()
                .and_hms_nano_opt(23, 59, 59, 1_000_000_000)
                .unwrap(),
        );
        let midnight = Time::<UTC>::from_gregorian(
            NaiveDate::from_ymd_opt(2017, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
        );
        let midnight_tai: Time<TAI> = midnight.clone().into();

        assert!(leap_second < midnight_tai);
        assert!(midnight_tai > leap_second);
    }

    #[test]
    fn test_fractional_scale_shift() {
        let time = Time::<UTC>::new(TimeDelta::seconds(10));
        let shifted = time.shift_scale_secs::<UTC>(-1.422818);

        assert_eq!(shifted.value, TimeDelta::new(8, 577_182_000).unwrap());
    }

    #[test]
    fn test_jd() {
        assert_eq!(Time::<UTC>::from_jd(0.).jd(), 0.0);
    }

    #[test]
    fn test_split_jd() {
        assert_eq!(Time::<UTC>::from_jd(0.).split_jd(), (0.0, 0.0));
    }

    #[test]
    fn test_from_split_jd() {
        let time = Time::<UTC>::from_split_jd(2451545.0, 0.75);
        assert_eq!(time.value, TimeDelta::seconds(2451545 * 86400 + 64800));
    }

    #[test]
    fn test_mjd() {
        assert_eq!(Time::<UTC>::from_jd(0.).mjd(), -JD_TO_MJD);
    }

    #[test]
    fn test_gregorian() {
        let time = Time::<UTC>::from_split_jd(2_451_545.0, 0.0);
        let expected = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap();

        assert_eq!(time.gregorian(), expected);
    }

    #[test]
    fn test_from_gregorian() {
        let gregorian = NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_nano_opt(12, 0, 0, 123_456_789)
            .unwrap();
        let time = Time::<UTC>::from_gregorian(gregorian);

        assert_eq!(time.gregorian(), gregorian);
        assert_eq!(
            time.value,
            TimeDelta::seconds(2_451_545_i64 * 86_400) + TimeDelta::nanoseconds(123_456_789)
        );
    }

    #[test]
    fn test_utc_gregorian_leap_second() {
        let gregorian = NaiveDate::from_ymd_opt(2016, 12, 31)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 1_000_000_000)
            .unwrap();
        let time = Time::<UTC>::from_gregorian(gregorian);

        assert_eq!(time.gregorian(), gregorian);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let time = Time::<UTC>::from_split_jd(2_451_545.0, 0.123_456_789);
        let json = serde_json::to_string(&time).unwrap();

        assert_eq!(serde_json::from_str::<Time<UTC>>(&json).unwrap(), time);
    }
}

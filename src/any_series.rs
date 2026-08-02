use crate::{scales, AnyTime, TimeScale, TimeSeries};
use chrono::TimeDelta;

/// A collection of time values whose common scale is selected at runtime.
///
/// Each variant stores a homogeneous [`TimeSeries`] and records its scale once
/// for the whole collection. Prefer `TimeSeries<S>` when the scale is known at
/// compile time.
///
/// # Examples
///
/// ```
/// use anytime::{AnyTimeSeries, Time, TimeSeries, scales::{TAI, UTC}};
///
/// let tai = TimeSeries::new(vec![Time::<TAI>::from_jd(2_451_545.0)]);
/// let series: AnyTimeSeries = tai.into();
/// let utc: TimeSeries<UTC> = series.into();
///
/// assert_eq!(utc.len(), 1);
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AnyTimeSeries {
    /// A series on the BeiDou time scale.
    BDT(TimeSeries<scales::BDT>),
    /// A series on the GLONASS time scale.
    GLONASST(TimeSeries<scales::GLONASST>),
    /// A series on the GPS time scale.
    GPST(TimeSeries<scales::GPST>),
    /// A series on the Galileo System Time scale.
    GST(TimeSeries<scales::GST>),
    /// A series on the QZSS time scale.
    QZZST(TimeSeries<scales::QZZST>),
    /// A series on the International Atomic Time scale.
    TAI(TimeSeries<scales::TAI>),
    /// A series on the Barycentric Coordinate Time scale.
    TCB(TimeSeries<scales::TCB>),
    /// A series on the Geocentric Coordinate Time scale.
    TCG(TimeSeries<scales::TCG>),
    /// A series on the Barycentric Dynamical Time scale.
    TDB(TimeSeries<scales::TDB>),
    /// A series on the Terrestrial Time scale.
    TT(TimeSeries<scales::TT>),
    /// A series on the Universal Time 1 scale.
    UT1(TimeSeries<scales::UT1>),
    /// A series on the Coordinated Universal Time scale.
    UTC(TimeSeries<scales::UTC>),
}

impl AnyTimeSeries {
    /// Creates a series by converting all values to `scale`.
    ///
    /// This is useful when input values have runtime-selected scales but the
    /// resulting collection must use one common scale.
    pub fn from_times(times: impl IntoIterator<Item = AnyTime>, scale: TimeScale) -> Self {
        match scale {
            TimeScale::BDT => {
                Self::BDT(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::GLONASST => {
                Self::GLONASST(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::GPST => {
                Self::GPST(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::GST => {
                Self::GST(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::QZZST => {
                Self::QZZST(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::TAI => {
                Self::TAI(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::TCB => {
                Self::TCB(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::TCG => {
                Self::TCG(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::TDB => {
                Self::TDB(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::TT => Self::TT(TimeSeries::new(times.into_iter().map(Into::into).collect())),
            TimeScale::UT1 => {
                Self::UT1(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
            TimeScale::UTC => {
                Self::UTC(TimeSeries::new(times.into_iter().map(Into::into).collect()))
            }
        }
    }

    /// Returns the common scale of the series.
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

    /// Returns the number of time values in the series.
    pub fn len(&self) -> usize {
        match self {
            Self::BDT(series) => series.len(),
            Self::GLONASST(series) => series.len(),
            Self::GPST(series) => series.len(),
            Self::GST(series) => series.len(),
            Self::QZZST(series) => series.len(),
            Self::TAI(series) => series.len(),
            Self::TCB(series) => series.len(),
            Self::TCG(series) => series.len(),
            Self::TDB(series) => series.len(),
            Self::TT(series) => series.len(),
            Self::UT1(series) => series.len(),
            Self::UTC(series) => series.len(),
        }
    }

    /// Returns whether the series contains no time values.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::BDT(series) => series.is_empty(),
            Self::GLONASST(series) => series.is_empty(),
            Self::GPST(series) => series.is_empty(),
            Self::GST(series) => series.is_empty(),
            Self::QZZST(series) => series.is_empty(),
            Self::TAI(series) => series.is_empty(),
            Self::TCB(series) => series.is_empty(),
            Self::TCG(series) => series.is_empty(),
            Self::TDB(series) => series.is_empty(),
            Self::TT(series) => series.is_empty(),
            Self::UT1(series) => series.is_empty(),
            Self::UTC(series) => series.is_empty(),
        }
    }

    /// Returns the elapsed physical duration from the earliest to latest time.
    pub fn duration(&self) -> TimeDelta {
        match self {
            Self::BDT(series) => series.duration(),
            Self::GLONASST(series) => series.duration(),
            Self::GPST(series) => series.duration(),
            Self::GST(series) => series.duration(),
            Self::QZZST(series) => series.duration(),
            Self::TAI(series) => series.duration(),
            Self::TCB(series) => series.duration(),
            Self::TCG(series) => series.duration(),
            Self::TDB(series) => series.duration(),
            Self::TT(series) => series.duration(),
            Self::UT1(series) => series.duration(),
            Self::UTC(series) => series.duration(),
        }
    }

    /// Returns the first time in the series.
    pub fn first(&self) -> Option<AnyTime> {
        self.iter().next()
    }

    /// Returns the final time in the series.
    pub fn last(&self) -> Option<AnyTime> {
        match self {
            Self::BDT(series) => series.last().cloned().map(Into::into),
            Self::GLONASST(series) => series.last().cloned().map(Into::into),
            Self::GPST(series) => series.last().cloned().map(Into::into),
            Self::GST(series) => series.last().cloned().map(Into::into),
            Self::QZZST(series) => series.last().cloned().map(Into::into),
            Self::TAI(series) => series.last().cloned().map(Into::into),
            Self::TCB(series) => series.last().cloned().map(Into::into),
            Self::TCG(series) => series.last().cloned().map(Into::into),
            Self::TDB(series) => series.last().cloned().map(Into::into),
            Self::TT(series) => series.last().cloned().map(Into::into),
            Self::UT1(series) => series.last().cloned().map(Into::into),
            Self::UTC(series) => series.last().cloned().map(Into::into),
        }
    }

    /// Iterates over the time values in their stored order.
    ///
    /// Values are returned as owned [`AnyTime`] instances because the concrete
    /// type of a borrowed [`crate::Time`] depends on the runtime-selected scale.
    pub fn iter(&self) -> Box<dyn Iterator<Item = AnyTime> + '_> {
        match self {
            Self::BDT(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::GLONASST(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::GPST(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::GST(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::QZZST(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::TAI(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::TCB(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::TCG(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::TDB(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::TT(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::UT1(series) => Box::new(series.iter().cloned().map(Into::into)),
            Self::UTC(series) => Box::new(series.iter().cloned().map(Into::into)),
        }
    }
}

impl IntoIterator for AnyTimeSeries {
    type Item = AnyTime;
    type IntoIter = Box<dyn Iterator<Item = AnyTime>>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::BDT(series) => Box::new(series.into_iter().map(Into::into)),
            Self::GLONASST(series) => Box::new(series.into_iter().map(Into::into)),
            Self::GPST(series) => Box::new(series.into_iter().map(Into::into)),
            Self::GST(series) => Box::new(series.into_iter().map(Into::into)),
            Self::QZZST(series) => Box::new(series.into_iter().map(Into::into)),
            Self::TAI(series) => Box::new(series.into_iter().map(Into::into)),
            Self::TCB(series) => Box::new(series.into_iter().map(Into::into)),
            Self::TCG(series) => Box::new(series.into_iter().map(Into::into)),
            Self::TDB(series) => Box::new(series.into_iter().map(Into::into)),
            Self::TT(series) => Box::new(series.into_iter().map(Into::into)),
            Self::UT1(series) => Box::new(series.into_iter().map(Into::into)),
            Self::UTC(series) => Box::new(series.into_iter().map(Into::into)),
        }
    }
}

macro_rules! impl_any_time_series_conversions {
    ($scale:ty, $variant:ident) => {
        impl From<TimeSeries<$scale>> for AnyTimeSeries {
            fn from(series: TimeSeries<$scale>) -> Self {
                Self::$variant(series)
            }
        }

        impl From<AnyTimeSeries> for TimeSeries<$scale> {
            fn from(series: AnyTimeSeries) -> Self {
                match series {
                    AnyTimeSeries::BDT(series) => series.into(),
                    AnyTimeSeries::GLONASST(series) => series.into(),
                    AnyTimeSeries::GPST(series) => series.into(),
                    AnyTimeSeries::GST(series) => series.into(),
                    AnyTimeSeries::QZZST(series) => series.into(),
                    AnyTimeSeries::TAI(series) => series.into(),
                    AnyTimeSeries::TCB(series) => series.into(),
                    AnyTimeSeries::TCG(series) => series.into(),
                    AnyTimeSeries::TDB(series) => series.into(),
                    AnyTimeSeries::TT(series) => series.into(),
                    AnyTimeSeries::UT1(series) => series.into(),
                    AnyTimeSeries::UTC(series) => series.into(),
                }
            }
        }
    };
}

impl_any_time_series_conversions!(scales::BDT, BDT);
impl_any_time_series_conversions!(scales::GLONASST, GLONASST);
impl_any_time_series_conversions!(scales::GPST, GPST);
impl_any_time_series_conversions!(scales::GST, GST);
impl_any_time_series_conversions!(scales::QZZST, QZZST);
impl_any_time_series_conversions!(scales::TAI, TAI);
impl_any_time_series_conversions!(scales::TCB, TCB);
impl_any_time_series_conversions!(scales::TCG, TCG);
impl_any_time_series_conversions!(scales::TDB, TDB);
impl_any_time_series_conversions!(scales::TT, TT);
impl_any_time_series_conversions!(scales::UT1, UT1);
impl_any_time_series_conversions!(scales::UTC, UTC);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        scales::{TAI, UTC},
        Time,
    };

    fn tai(seconds: i64) -> Time<TAI> {
        Time::new(TimeDelta::seconds(seconds))
    }

    #[test]
    fn converts_to_and_from_typed_series() {
        let series: AnyTimeSeries = TimeSeries::new(vec![tai(0), tai(1)]).into();
        let utc: TimeSeries<UTC> = series.into();

        assert_eq!(utc.len(), 2);
        assert_eq!(utc.duration(), TimeDelta::seconds(1));
    }

    #[test]
    fn exposes_common_series_operations() {
        let series: AnyTimeSeries = TimeSeries::new(vec![tai(2), tai(0), tai(1)]).into();

        assert_eq!(series.scale(), TimeScale::TAI);
        assert_eq!(series.len(), 3);
        assert!(!series.is_empty());
        assert_eq!(series.duration(), TimeDelta::seconds(2));
        assert_eq!(series.first(), Some(AnyTime::TAI(tai(2))));
        assert_eq!(series.last(), Some(AnyTime::TAI(tai(1))));
        assert_eq!(
            series.iter().collect::<Vec<_>>(),
            vec![
                AnyTime::TAI(tai(2)),
                AnyTime::TAI(tai(0)),
                AnyTime::TAI(tai(1))
            ]
        );
        assert_eq!(series.into_iter().count(), 3);
    }

    #[test]
    fn normalizes_values_to_the_requested_scale() {
        let utc: Time<UTC> = tai(0).into();
        let series = AnyTimeSeries::from_times(
            vec![AnyTime::TAI(tai(0)), AnyTime::UTC(utc)],
            TimeScale::TAI,
        );

        assert_eq!(series.scale(), TimeScale::TAI);
        assert_eq!(
            series.iter().collect::<Vec<_>>(),
            vec![AnyTime::TAI(tai(0)), AnyTime::TAI(tai(0))]
        );
    }

    #[test]
    fn preserves_scale_for_empty_series() {
        let series = AnyTimeSeries::from_times(Vec::new(), TimeScale::UTC);

        assert_eq!(series.scale(), TimeScale::UTC);
        assert!(series.is_empty());
        assert_eq!(series.duration(), TimeDelta::zero());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let series: AnyTimeSeries = TimeSeries::new(vec![tai(0), tai(1)]).into();
        let json = serde_json::to_string(&series).unwrap();
        let deserialized = serde_json::from_str::<AnyTimeSeries>(&json).unwrap();

        assert_eq!(
            deserialized.iter().collect::<Vec<_>>(),
            series.into_iter().collect::<Vec<_>>()
        );
    }
}

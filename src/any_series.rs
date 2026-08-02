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

/// An iterator that borrows an [`AnyTimeSeries`] and yields [`AnyTime`] values.
pub struct AnyTimeSeriesIter<'a>(AnyTimeSeriesIterInner<'a>);

enum AnyTimeSeriesIterInner<'a> {
    BDT(std::slice::Iter<'a, crate::Time<scales::BDT>>),
    GLONASST(std::slice::Iter<'a, crate::Time<scales::GLONASST>>),
    GPST(std::slice::Iter<'a, crate::Time<scales::GPST>>),
    GST(std::slice::Iter<'a, crate::Time<scales::GST>>),
    QZZST(std::slice::Iter<'a, crate::Time<scales::QZZST>>),
    TAI(std::slice::Iter<'a, crate::Time<scales::TAI>>),
    TCB(std::slice::Iter<'a, crate::Time<scales::TCB>>),
    TCG(std::slice::Iter<'a, crate::Time<scales::TCG>>),
    TDB(std::slice::Iter<'a, crate::Time<scales::TDB>>),
    TT(std::slice::Iter<'a, crate::Time<scales::TT>>),
    UT1(std::slice::Iter<'a, crate::Time<scales::UT1>>),
    UTC(std::slice::Iter<'a, crate::Time<scales::UTC>>),
}

/// An owning iterator over values in an [`AnyTimeSeries`].
pub struct AnyTimeSeriesIntoIter(AnyTimeSeriesIntoIterInner);

enum AnyTimeSeriesIntoIterInner {
    BDT(std::vec::IntoIter<crate::Time<scales::BDT>>),
    GLONASST(std::vec::IntoIter<crate::Time<scales::GLONASST>>),
    GPST(std::vec::IntoIter<crate::Time<scales::GPST>>),
    GST(std::vec::IntoIter<crate::Time<scales::GST>>),
    QZZST(std::vec::IntoIter<crate::Time<scales::QZZST>>),
    TAI(std::vec::IntoIter<crate::Time<scales::TAI>>),
    TCB(std::vec::IntoIter<crate::Time<scales::TCB>>),
    TCG(std::vec::IntoIter<crate::Time<scales::TCG>>),
    TDB(std::vec::IntoIter<crate::Time<scales::TDB>>),
    TT(std::vec::IntoIter<crate::Time<scales::TT>>),
    UT1(std::vec::IntoIter<crate::Time<scales::UT1>>),
    UTC(std::vec::IntoIter<crate::Time<scales::UTC>>),
}

macro_rules! with_series {
    ($value:expr, |$series:ident| $body:expr) => {
        match $value {
            AnyTimeSeries::BDT($series) => $body,
            AnyTimeSeries::GLONASST($series) => $body,
            AnyTimeSeries::GPST($series) => $body,
            AnyTimeSeries::GST($series) => $body,
            AnyTimeSeries::QZZST($series) => $body,
            AnyTimeSeries::TAI($series) => $body,
            AnyTimeSeries::TCB($series) => $body,
            AnyTimeSeries::TCG($series) => $body,
            AnyTimeSeries::TDB($series) => $body,
            AnyTimeSeries::TT($series) => $body,
            AnyTimeSeries::UT1($series) => $body,
            AnyTimeSeries::UTC($series) => $body,
        }
    };
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

    /// Converts every value to `scale` and returns a runtime-scale series.
    ///
    /// Stored order and length are preserved. When the series already uses the
    /// requested scale, it is returned unchanged.
    pub fn convert(self, scale: TimeScale) -> Self {
        if self.scale() == scale {
            return self;
        }

        Self::from_times(self, scale)
    }

    /// Returns the number of time values in the series.
    pub fn len(&self) -> usize {
        with_series!(self, |series| series.len())
    }

    /// Returns whether the series contains no time values.
    pub fn is_empty(&self) -> bool {
        with_series!(self, |series| series.is_empty())
    }

    /// Returns the elapsed physical duration from the earliest to latest time.
    pub fn duration(&self) -> TimeDelta {
        with_series!(self, |series| series.duration())
    }

    /// Returns the first time in the series.
    pub fn first(&self) -> Option<AnyTime> {
        with_series!(self, |series| series.first().cloned().map(Into::into))
    }

    /// Returns the final time in the series.
    pub fn last(&self) -> Option<AnyTime> {
        with_series!(self, |series| series.last().cloned().map(Into::into))
    }

    /// Sorts the series in ascending physical order.
    pub fn sort(&mut self) {
        with_series!(self, |series| series.sort())
    }

    /// Reverses the stored order of the series.
    pub fn reverse(&mut self) {
        with_series!(self, |series| series.reverse())
    }

    /// Appends a time after converting it to the series' common scale.
    pub fn push(&mut self, time: AnyTime) {
        with_series!(self, |series| series.push(time.into()))
    }

    /// Consumes the series and returns its values in stored order.
    pub fn into_vec(self) -> Vec<AnyTime> {
        self.into_iter().collect()
    }

    /// Returns the time at `index`, or `None` when the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<AnyTime> {
        with_series!(self, |series| series.get(index).cloned().map(Into::into))
    }

    /// Returns the earliest physical time in the series.
    ///
    /// Stored order is ignored. When the earliest time occurs more than once,
    /// the first occurrence is returned.
    pub fn earliest(&self) -> Option<AnyTime> {
        with_series!(self, |series| series.earliest().cloned().map(Into::into))
    }

    /// Returns the latest physical time in the series.
    ///
    /// Stored order is ignored. When the latest time occurs more than once,
    /// the first occurrence is returned.
    pub fn latest(&self) -> Option<AnyTime> {
        with_series!(self, |series| series.latest().cloned().map(Into::into))
    }

    /// Returns whether the series contains the same physical instant as `time`.
    pub fn contains(&self, time: &AnyTime) -> bool {
        self.iter().any(|candidate| candidate == *time)
    }

    /// Returns whether the times are in ascending physical order.
    pub fn is_sorted(&self) -> bool {
        with_series!(self, |series| series.is_sorted())
    }

    /// Returns the stored time nearest to `time` as a physical instant.
    ///
    /// When two values are equally near, the first one in stored order is
    /// returned.
    pub fn nearest(&self, time: &AnyTime) -> Option<AnyTime> {
        self.iter()
            .min_by_key(|candidate| (candidate.clone() - time.clone()).abs())
    }

    /// Iterates over times within the inclusive physical interval.
    ///
    /// Values are yielded in stored order and duplicates are preserved. The
    /// iterator is empty when `start` is later than `end`.
    pub fn within(&self, start: &AnyTime, end: &AnyTime) -> impl Iterator<Item = AnyTime> + '_ {
        let start = start.clone();
        let end = end.clone();
        self.iter()
            .filter(move |time| time >= &start && time <= &end)
    }

    /// Iterates over the time values in their stored order.
    ///
    /// Values are returned as owned [`AnyTime`] instances because the concrete
    /// type of a borrowed [`crate::Time`] depends on the runtime-selected scale.
    pub fn iter(&self) -> AnyTimeSeriesIter<'_> {
        AnyTimeSeriesIter(match self {
            Self::BDT(series) => AnyTimeSeriesIterInner::BDT(series.iter()),
            Self::GLONASST(series) => AnyTimeSeriesIterInner::GLONASST(series.iter()),
            Self::GPST(series) => AnyTimeSeriesIterInner::GPST(series.iter()),
            Self::GST(series) => AnyTimeSeriesIterInner::GST(series.iter()),
            Self::QZZST(series) => AnyTimeSeriesIterInner::QZZST(series.iter()),
            Self::TAI(series) => AnyTimeSeriesIterInner::TAI(series.iter()),
            Self::TCB(series) => AnyTimeSeriesIterInner::TCB(series.iter()),
            Self::TCG(series) => AnyTimeSeriesIterInner::TCG(series.iter()),
            Self::TDB(series) => AnyTimeSeriesIterInner::TDB(series.iter()),
            Self::TT(series) => AnyTimeSeriesIterInner::TT(series.iter()),
            Self::UT1(series) => AnyTimeSeriesIterInner::UT1(series.iter()),
            Self::UTC(series) => AnyTimeSeriesIterInner::UTC(series.iter()),
        })
    }
}

impl Iterator for AnyTimeSeriesIter<'_> {
    type Item = AnyTime;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            AnyTimeSeriesIterInner::BDT(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::GLONASST(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::GPST(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::GST(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::QZZST(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::TAI(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::TCB(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::TCG(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::TDB(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::TT(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::UT1(iter) => iter.next().cloned().map(Into::into),
            AnyTimeSeriesIterInner::UTC(iter) => iter.next().cloned().map(Into::into),
        }
    }
}

impl IntoIterator for AnyTimeSeries {
    type Item = AnyTime;
    type IntoIter = AnyTimeSeriesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        AnyTimeSeriesIntoIter(match self {
            Self::BDT(series) => AnyTimeSeriesIntoIterInner::BDT(series.into_iter()),
            Self::GLONASST(series) => AnyTimeSeriesIntoIterInner::GLONASST(series.into_iter()),
            Self::GPST(series) => AnyTimeSeriesIntoIterInner::GPST(series.into_iter()),
            Self::GST(series) => AnyTimeSeriesIntoIterInner::GST(series.into_iter()),
            Self::QZZST(series) => AnyTimeSeriesIntoIterInner::QZZST(series.into_iter()),
            Self::TAI(series) => AnyTimeSeriesIntoIterInner::TAI(series.into_iter()),
            Self::TCB(series) => AnyTimeSeriesIntoIterInner::TCB(series.into_iter()),
            Self::TCG(series) => AnyTimeSeriesIntoIterInner::TCG(series.into_iter()),
            Self::TDB(series) => AnyTimeSeriesIntoIterInner::TDB(series.into_iter()),
            Self::TT(series) => AnyTimeSeriesIntoIterInner::TT(series.into_iter()),
            Self::UT1(series) => AnyTimeSeriesIntoIterInner::UT1(series.into_iter()),
            Self::UTC(series) => AnyTimeSeriesIntoIterInner::UTC(series.into_iter()),
        })
    }
}

impl Iterator for AnyTimeSeriesIntoIter {
    type Item = AnyTime;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            AnyTimeSeriesIntoIterInner::BDT(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::GLONASST(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::GPST(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::GST(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::QZZST(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::TAI(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::TCB(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::TCG(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::TDB(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::TT(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::UT1(iter) => iter.next().map(Into::into),
            AnyTimeSeriesIntoIterInner::UTC(iter) => iter.next().map(Into::into),
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

    fn series_for_each_scale() -> Vec<AnyTimeSeries> {
        vec![
            AnyTimeSeries::BDT(TimeSeries::new(vec![
                Time::<scales::BDT>::new(TimeDelta::zero()),
                Time::<scales::BDT>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::GLONASST(TimeSeries::new(vec![
                Time::<scales::GLONASST>::new(TimeDelta::zero()),
                Time::<scales::GLONASST>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::GPST(TimeSeries::new(vec![
                Time::<scales::GPST>::new(TimeDelta::zero()),
                Time::<scales::GPST>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::GST(TimeSeries::new(vec![
                Time::<scales::GST>::new(TimeDelta::zero()),
                Time::<scales::GST>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::QZZST(TimeSeries::new(vec![
                Time::<scales::QZZST>::new(TimeDelta::zero()),
                Time::<scales::QZZST>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::TAI(TimeSeries::new(vec![tai(0), tai(1)])),
            AnyTimeSeries::TCB(TimeSeries::new(vec![
                Time::<scales::TCB>::new(TimeDelta::zero()),
                Time::<scales::TCB>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::TCG(TimeSeries::new(vec![
                Time::<scales::TCG>::new(TimeDelta::zero()),
                Time::<scales::TCG>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::TDB(TimeSeries::new(vec![
                Time::<scales::TDB>::new(TimeDelta::zero()),
                Time::<scales::TDB>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::TT(TimeSeries::new(vec![
                Time::<scales::TT>::new(TimeDelta::zero()),
                Time::<scales::TT>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::UT1(TimeSeries::new(vec![
                Time::<scales::UT1>::new(TimeDelta::zero()),
                Time::<scales::UT1>::new(TimeDelta::seconds(1)),
            ])),
            AnyTimeSeries::UTC(TimeSeries::new(vec![
                Time::<scales::UTC>::new(TimeDelta::zero()),
                Time::<scales::UTC>::new(TimeDelta::seconds(1)),
            ])),
        ]
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
        let iter: AnyTimeSeriesIter<'_> = series.iter();
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![
                AnyTime::TAI(tai(2)),
                AnyTime::TAI(tai(0)),
                AnyTime::TAI(tai(1))
            ]
        );
        let into_iter: AnyTimeSeriesIntoIter = series.into_iter();
        assert_eq!(into_iter.count(), 3);
    }

    #[test]
    fn supports_collection_operations() {
        let mut series: AnyTimeSeries = TimeSeries::new(vec![tai(2), tai(0)]).into();

        series.push(AnyTime::UTC(tai(1).utc()));
        assert_eq!(series.scale(), TimeScale::TAI);
        assert_eq!(
            series.iter().collect::<Vec<_>>(),
            vec![
                AnyTime::TAI(tai(2)),
                AnyTime::TAI(tai(0)),
                AnyTime::TAI(tai(1))
            ]
        );

        series.sort();
        series.reverse();
        assert_eq!(
            series.into_vec(),
            vec![
                AnyTime::TAI(tai(2)),
                AnyTime::TAI(tai(1)),
                AnyTime::TAI(tai(0))
            ]
        );
    }

    #[test]
    fn exposes_read_only_queries() {
        let series: AnyTimeSeries =
            TimeSeries::new(vec![tai(3), tai(1), tai(2), tai(1), tai(0)]).into();

        assert_eq!(series.get(1), Some(AnyTime::TAI(tai(1))));
        assert_eq!(series.get(5), None);
        assert_eq!(series.earliest(), Some(AnyTime::TAI(tai(0))));
        assert_eq!(series.latest(), Some(AnyTime::TAI(tai(3))));
        assert!(series.contains(&AnyTime::UTC(tai(2).utc())));
        assert!(!series.contains(&AnyTime::UTC(tai(4).utc())));
        assert!(!series.is_sorted());
        assert_eq!(
            series.nearest(&AnyTime::UTC(tai(1).utc())),
            Some(AnyTime::TAI(tai(1)))
        );
        assert_eq!(
            series
                .within(&AnyTime::UTC(tai(1).utc()), &AnyTime::TAI(tai(2)))
                .collect::<Vec<_>>(),
            vec![
                AnyTime::TAI(tai(1)),
                AnyTime::TAI(tai(2)),
                AnyTime::TAI(tai(1))
            ]
        );
        assert_eq!(
            series
                .within(&AnyTime::TAI(tai(2)), &AnyTime::TAI(tai(1)))
                .count(),
            0
        );
    }

    #[test]
    fn every_variant_supports_common_operations_and_runtime_conversion() {
        for series in series_for_each_scale() {
            let scale = series.scale();

            assert_eq!(series.len(), 2);
            assert!(!series.is_empty());
            assert_eq!(series.first().unwrap().scale(), scale);
            assert_eq!(series.last().unwrap().scale(), scale);
            assert!(series.duration() > TimeDelta::zero());

            let mut iter = series.iter();
            assert_eq!(iter.next().unwrap().scale(), scale);
            assert_eq!(iter.count(), 1);

            let tai: TimeSeries<TAI> = series.into();
            assert_eq!(tai.len(), 2);
        }

        for series in series_for_each_scale() {
            assert_eq!(series.into_iter().count(), 2);
        }
    }

    #[test]
    fn normalizes_values_to_every_requested_scale() {
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
        let utc: Time<UTC> = tai(0).into();

        for scale in scales {
            let series = AnyTimeSeries::from_times(
                vec![AnyTime::TAI(tai(0)), AnyTime::UTC(utc.clone())],
                scale,
            );

            assert_eq!(series.scale(), scale);
            assert_eq!(series.len(), 2);
            assert!(series.iter().all(|time| time.scale() == scale));
        }
    }

    #[test]
    fn typed_series_converts_to_every_runtime_variant() {
        macro_rules! assert_runtime_variant {
            ($scale:ident) => {{
                let series = TimeSeries::<scales::$scale>::new(vec![Time::<scales::$scale>::new(
                    TimeDelta::zero(),
                )]);
                let any: AnyTimeSeries = series.into();

                assert_eq!(any.scale(), TimeScale::$scale);
            }};
        }

        assert_runtime_variant!(BDT);
        assert_runtime_variant!(GLONASST);
        assert_runtime_variant!(GPST);
        assert_runtime_variant!(GST);
        assert_runtime_variant!(QZZST);
        assert_runtime_variant!(TAI);
        assert_runtime_variant!(TCB);
        assert_runtime_variant!(TCG);
        assert_runtime_variant!(TDB);
        assert_runtime_variant!(TT);
        assert_runtime_variant!(UT1);
        assert_runtime_variant!(UTC);
    }

    #[test]
    fn converts_to_a_runtime_selected_scale() {
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

        for scale in scales {
            let series: AnyTimeSeries = TimeSeries::new(vec![tai(0), tai(1)]).into();
            let converted = series.convert(scale);

            assert_eq!(converted.scale(), scale);
            assert_eq!(converted.len(), 2);
            assert_eq!(
                converted.iter().collect::<Vec<_>>(),
                vec![
                    AnyTime::TAI(tai(0)).convert(scale),
                    AnyTime::TAI(tai(1)).convert(scale)
                ]
            );
        }
    }

    #[test]
    fn convert_preserves_an_empty_series_scale() {
        let series = AnyTimeSeries::from_times(Vec::new(), TimeScale::TAI);

        let utc = series.convert(TimeScale::UTC);
        assert_eq!(utc.scale(), TimeScale::UTC);
        assert!(utc.is_empty());

        let utc = utc.convert(TimeScale::UTC);
        assert_eq!(utc.scale(), TimeScale::UTC);
        assert!(utc.is_empty());
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
        for series in series_for_each_scale() {
            let scale = series.scale();
            let json = serde_json::to_string(&series).unwrap();
            let deserialized = serde_json::from_str::<AnyTimeSeries>(&json).unwrap();

            assert_eq!(deserialized.scale(), scale);
            assert_eq!(
                deserialized.iter().collect::<Vec<_>>(),
                series.into_iter().collect::<Vec<_>>()
            );
        }
    }
}

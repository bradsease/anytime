use crate::{scales::TAI, Time};
use chrono::TimeDelta;

/// A collection of time values associated with a specific time scale.
///
/// Values retain the order in which they were supplied. Use [`Self::duration`]
/// to measure the physical span between the earliest and latest values,
/// regardless of their stored order.
///
/// # Examples
///
/// ```
/// use anytime::{Time, TimeSeries, scales::TAI};
/// use chrono::TimeDelta;
///
/// let series = TimeSeries::from_range(
///     Time::<TAI>::from_jd(2_451_545.0),
///     Time::<TAI>::from_jd(2_451_548.0),
///     TimeDelta::days(1),
/// );
/// assert_eq!(series.len(), 3);
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = ""))]
pub struct TimeSeries<S> {
    times: Vec<Time<S>>,
}

/// A lazily generated range of times with an exclusive end bound.
///
/// Create one with [`TimeSeries::range_iter`]. The iterator yields the start
/// value when it is on the correct side of the end value, then advances by its
/// step until the exclusive bound is reached.
#[derive(Debug)]
pub struct TimeSeriesRange<S> {
    next: Option<Time<S>>,
    end: Time<S>,
    step: TimeDelta,
}

impl<S> TimeSeries<S> {
    /// Creates a series from time values in their supplied order.
    ///
    /// # Examples
    ///
    /// ```
    /// use anytime::{Time, TimeSeries, scales::TAI};
    ///
    /// let series = TimeSeries::new(vec![
    ///     Time::<TAI>::from_jd(2_451_545.0),
    ///     Time::<TAI>::from_jd(2_451_546.0),
    /// ]);
    /// assert_eq!(series.len(), 2);
    /// ```
    pub fn new(times: Vec<Time<S>>) -> Self {
        Self { times }
    }

    /// Returns the number of time values in the series.
    pub fn len(&self) -> usize {
        self.times.len()
    }

    /// Returns whether the series contains no time values.
    pub fn is_empty(&self) -> bool {
        self.times.is_empty()
    }

    /// Returns the first time in the series.
    pub fn first(&self) -> Option<&Time<S>> {
        self.times.first()
    }

    /// Returns the final time in the series.
    pub fn last(&self) -> Option<&Time<S>> {
        self.times.last()
    }

    /// Returns the time at `index`, or `None` when the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Time<S>> {
        self.times.get(index)
    }

    /// Returns the earliest physical time in the series.
    ///
    /// Stored order is ignored. When the earliest time occurs more than once,
    /// the first occurrence is returned.
    pub fn earliest(&self) -> Option<&Time<S>> {
        self.times.iter().min()
    }

    /// Returns the latest physical time in the series.
    ///
    /// Stored order is ignored. When the latest time occurs more than once,
    /// the first occurrence is returned.
    pub fn latest(&self) -> Option<&Time<S>> {
        self.times
            .iter()
            .reduce(|latest, time| if time > latest { time } else { latest })
    }

    /// Returns whether the series contains the same physical instant as `time`.
    pub fn contains<T>(&self, time: &Time<T>) -> bool
    where
        Time<T>: Into<Time<S>>,
    {
        self.times.iter().any(|candidate| candidate == time)
    }

    /// Returns whether the times are in ascending physical order.
    pub fn is_sorted(&self) -> bool {
        self.times.windows(2).all(|pair| pair[0] <= pair[1])
    }

    /// Returns the stored time nearest to `time` as a physical instant.
    ///
    /// When two values are equally near, the first one in stored order is
    /// returned.
    pub fn nearest<T>(&self, time: &Time<T>) -> Option<&Time<S>>
    where
        Time<S>: Into<Time<TAI>>,
        Time<T>: Into<Time<TAI>>,
    {
        let time: Time<TAI> = time.clone().into();
        self.times.iter().min_by_key(|candidate| {
            let candidate: Time<TAI> = (*candidate).clone().into();
            (candidate.value - time.value).abs()
        })
    }

    /// Iterates over times within the inclusive physical interval.
    ///
    /// Values are yielded in stored order and duplicates are preserved. The
    /// iterator is empty when `start` is later than `end`.
    pub fn within<T, U>(
        &self,
        start: &Time<T>,
        end: &Time<U>,
    ) -> impl Iterator<Item = &Time<S>> + '_
    where
        Time<T>: Into<Time<S>>,
        Time<U>: Into<Time<S>>,
    {
        let start: Time<S> = start.clone().into();
        let end: Time<S> = end.clone().into();
        self.times
            .iter()
            .filter(move |time| *time >= &start && *time <= &end)
    }

    /// Returns the time values in their stored order.
    pub fn as_slice(&self) -> &[Time<S>] {
        &self.times
    }

    /// Iterates over the time values in their stored order.
    pub fn iter(&self) -> std::slice::Iter<'_, Time<S>> {
        self.times.iter()
    }

    /// Returns the elapsed physical duration from the earliest to latest time.
    ///
    /// Empty and singleton series have zero duration.
    pub fn duration(&self) -> TimeDelta
    where
        Time<S>: Into<Time<TAI>>,
    {
        let Some(first) = self.times.first() else {
            return TimeDelta::zero();
        };

        let (earliest, latest) =
            self.times
                .iter()
                .skip(1)
                .fold((first, first), |(earliest, latest), time| {
                    (
                        if time < earliest { time } else { earliest },
                        if time > latest { time } else { latest },
                    )
                });

        latest.clone() - earliest.clone()
    }
}

impl<S, T> From<Vec<Time<S>>> for TimeSeries<T>
where
    Time<S>: Into<Time<T>>,
{
    fn from(times: Vec<Time<S>>) -> Self {
        Self::new(times.into_iter().map(|time| time.into()).collect())
    }
}

impl<S> From<Vec<crate::AnyTime>> for TimeSeries<S>
where
    Time<S>: From<crate::AnyTime>,
{
    fn from(times: Vec<crate::AnyTime>) -> Self {
        Self::new(times.into_iter().map(Into::into).collect())
    }
}

impl<S> IntoIterator for TimeSeries<S> {
    type Item = Time<S>;
    type IntoIter = std::vec::IntoIter<Time<S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.times.into_iter()
    }
}

impl<S: crate::Scale> Iterator for TimeSeriesRange<S> {
    type Item = Time<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.take()?;
        self.next = current
            .value
            .checked_add(&self.step)
            .map(Time::new)
            .filter(|next| {
                if self.step > TimeDelta::zero() {
                    next < &self.end
                } else {
                    next > &self.end
                }
            });
        Some(current)
    }
}

impl<S: crate::Scale> TimeSeries<S> {
    /// Lazily generates a range with an exclusive end bound.
    ///
    /// Positive steps generate ascending ranges and negative steps generate
    /// descending ranges. A range whose step points away from its end is empty.
    ///
    /// # Panics
    ///
    /// Panics if `step` is zero.
    pub fn range_iter(start: Time<S>, end: Time<S>, step: TimeDelta) -> TimeSeriesRange<S> {
        assert_ne!(step, TimeDelta::zero(), "time series step must not be zero");

        let has_values = if step > TimeDelta::zero() {
            start < end
        } else {
            start > end
        };

        TimeSeriesRange {
            next: has_values.then_some(start),
            end,
            step,
        }
    }

    /// Creates a range with an exclusive end bound.
    ///
    /// This is the eager counterpart to [`Self::range_iter`].
    pub fn from_range(start: Time<S>, end: Time<S>, step: TimeDelta) -> Self {
        Self {
            times: Self::range_iter(start, end, step).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scales::{TAI, UTC};

    fn tai(seconds: i64) -> Time<TAI> {
        Time::new(TimeDelta::seconds(seconds))
    }

    #[test]
    fn new_preserves_time_order() {
        let empty = TimeSeries::<TAI>::new(vec![]);
        assert!(empty.is_empty());
        assert_eq!(empty.first(), None);
        assert_eq!(empty.last(), None);
        assert_eq!(empty.duration(), TimeDelta::zero());
        let series = TimeSeries::new(vec![tai(2), tai(0), tai(1)]);

        assert_eq!(series.as_slice(), &[tai(2), tai(0), tai(1)]);
        assert_eq!(series.duration(), TimeDelta::seconds(2));
    }

    #[test]
    fn queries_stored_and_physical_order() {
        let series = TimeSeries::new(vec![tai(2), tai(0), tai(1), tai(0)]);

        assert_eq!(series.get(1), Some(&tai(0)));
        assert_eq!(series.get(4), None);
        assert_eq!(series.earliest(), Some(&tai(0)));
        assert_eq!(series.latest(), Some(&tai(2)));
        assert!(series.contains(&tai(1).utc()));
        assert!(!series.contains(&tai(3).utc()));
        assert!(!series.is_sorted());
        assert!(TimeSeries::new(vec![tai(0), tai(0), tai(1)]).is_sorted());
    }

    #[test]
    fn nearest_uses_physical_distance_and_stored_order_for_ties() {
        let series = TimeSeries::new(vec![tai(2), tai(0), tai(4)]);

        assert_eq!(series.nearest(&tai(1).utc()), Some(&tai(2)));
        assert_eq!(series.nearest(&tai(4).utc()), Some(&tai(4)));
        assert_eq!(TimeSeries::<TAI>::new(vec![]).nearest(&tai(0)), None);
    }

    #[test]
    fn within_is_inclusive_and_preserves_stored_order() {
        let series = TimeSeries::new(vec![tai(3), tai(1), tai(2), tai(1), tai(0)]);

        assert_eq!(
            series
                .within(&tai(1).utc(), &tai(2))
                .cloned()
                .collect::<Vec<_>>(),
            vec![tai(1), tai(2), tai(1)]
        );
        assert_eq!(series.within(&tai(2), &tai(1)).count(), 0);
    }

    #[test]
    fn range_is_ascending_and_has_a_duration() {
        let series = TimeSeries::from_range(tai(0), tai(3), TimeDelta::seconds(1));

        assert_eq!(series.len(), 3);
        assert_eq!(series.duration(), TimeDelta::seconds(2));
        assert_eq!(series.as_slice(), &[tai(0), tai(1), tai(2)]);
    }

    #[test]
    fn range_allows_a_single_time() {
        let series = TimeSeries::from_range(tai(0), tai(1), TimeDelta::seconds(2));

        assert_eq!(series.len(), 1);
        assert_eq!(series.duration(), TimeDelta::zero());
        assert_eq!(series.first(), Some(&tai(0)));
        assert_eq!(series.last(), Some(&tai(0)));
    }

    #[test]
    fn range_iter_generates_times_lazily() {
        let mut range = TimeSeries::range_iter(tai(0), tai(3), TimeDelta::seconds(1));

        assert_eq!(range.next(), Some(tai(0)));
        assert_eq!(range.collect::<Vec<_>>(), vec![tai(1), tai(2)]);
    }

    #[test]
    fn range_is_empty_when_step_points_away_from_end() {
        let ascending = TimeSeries::from_range(tai(1), tai(0), TimeDelta::seconds(1));
        let descending = TimeSeries::from_range(tai(0), tai(1), TimeDelta::seconds(-1));

        assert!(ascending.is_empty());
        assert!(descending.is_empty());
    }

    #[test]
    fn range_supports_negative_steps() {
        let series = TimeSeries::from_range(tai(3), tai(0), TimeDelta::seconds(-1));

        assert_eq!(series.as_slice(), &[tai(3), tai(2), tai(1)]);
        assert_eq!(series.duration(), TimeDelta::seconds(2));
    }

    #[test]
    #[should_panic(expected = "time series step must not be zero")]
    fn range_rejects_zero_step() {
        TimeSeries::range_iter(tai(0), tai(1), TimeDelta::zero());
    }

    #[test]
    fn series_converts_scales() {
        let series = TimeSeries::new(vec![tai(0), tai(1)]);
        let utc: TimeSeries<UTC> = series.into();

        assert_eq!(utc.len(), 2);
        assert_eq!(utc.duration(), TimeDelta::seconds(1));
    }

    #[test]
    fn vec_of_times_converts_to_a_series_in_the_target_scale() {
        let utc: TimeSeries<UTC> = vec![tai(0), tai(1)].into();

        assert_eq!(utc.as_slice(), &[tai(0).utc(), tai(1).utc()]);
    }

    #[test]
    fn vec_of_any_times_converts_to_a_series_in_the_target_scale() {
        let utc: TimeSeries<UTC> =
            vec![crate::AnyTime::TAI(tai(0)), crate::AnyTime::TAI(tai(1))].into();

        assert_eq!(utc.as_slice(), &[tai(0).utc(), tai(1).utc()]);
    }

    #[test]
    fn series_iterates_by_reference_and_value() {
        let series = TimeSeries::new(vec![tai(0), tai(1)]);
        assert_eq!(series.iter().count(), 2);
        assert_eq!(series.into_iter().count(), 2);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let series = TimeSeries::new(vec![tai(0), tai(1)]);
        let json = serde_json::to_string(&series).unwrap();
        let deserialized = serde_json::from_str::<TimeSeries<TAI>>(&json).unwrap();

        assert_eq!(deserialized.as_slice(), series.as_slice());
    }
}

//! Astronomical time scales and conversions with nanosecond resolution.
//!
//! [`Time`] stores an instant with a type-level [`Scale`]. The scale marker
//! prevents accidental mixing of values while the conversion implementations
//! make changes of scale explicit:
//!
//! ```
//! use anytime::{Time, scales::{TAI, UTC}};
//!
//! let utc = Time::<UTC>::from_jd(2_451_545.0);
//! let tai: Time<TAI> = utc.into();
//! # assert_eq!(tai.utc().jd(), 2_451_545.0);
//! ```
//!
//! Values can also be compared and differenced directly, even when they use
//! different scales. Subtraction returns a `chrono::TimeDelta`, while
//! comparisons use standard ordering operators:
//!
//! ```
//! use anytime::{Time, scales::{TAI, UTC}};
//! use chrono::TimeDelta;
//!
//! let earlier = Time::<UTC>::from_jd(2_451_545.0);
//! let later: Time<TAI> = Time::<UTC>::from_jd(2_451_546.0).into();
//! let is_ordered = earlier < later;
//! let difference = later - earlier;
//! # assert!(is_ordered);
//! # assert_eq!(difference, TimeDelta::seconds(86_400));
//! ```
//!
//! A value can be formatted as a Gregorian date string, including sub-second
//! precision, through [`Time::gregorian`]:
//!
//! ```
//! use anytime::{Time, scales::UTC};
//! use chrono::NaiveDate;
//!
//! let date = NaiveDate::from_ymd_opt(2000, 1, 1)
//!     .unwrap()
//!     .and_hms_nano_opt(12, 0, 0, 123_456_789)
//!     .unwrap();
//! let date_string = Time::<UTC>::from_gregorian(date)
//!     .gregorian()
//!     .to_string();
//! # assert_eq!(date_string, "2000-01-01 12:00:00.123456789");
//! ```
//!
//! When the scale is selected at runtime, use [`AnyTime`]'s constructors with
//! [`TimeScale`]:
//!
//! ```
//! use anytime::{AnyTime, TimeScale};
//!
//! let utc = AnyTime::from_isot_str("2000-01-01T12:00:00", TimeScale::UTC).unwrap();
//! # assert_eq!(utc, AnyTime::from_jd(2_451_545.0, TimeScale::UTC));
//! ```
//!
//! The crate supports UTC, TAI, GPST, TT, TCG, TCB, TDB, and UT1. UTC conversions
//! use leap-second data built into the crate. UT1 conversions additionally use
//! Earth-orientation data loaded with [`load_finals2000a`].
//!
//! # Feature overview
//!
//! - Use [`Time::from_jd`], [`Time::from_mjd`], [`Time::from_split_jd`], or
//!   [`Time::from_gregorian`] to construct a typed value, or [`AnyTime`]'s
//!   runtime-scale constructors when the scale is chosen dynamically.
//! - Use [`Time::jd`], [`Time::mjd`], [`Time::split_jd`], or
//!   [`Time::gregorian`] to inspect a value.
//! - Use the `From`/`Into` implementations or [`Time::utc`], [`Time::tai`], and
//!   the other scale methods to convert values.
//! - Use [`TimeSeries`] for stored collections or [`TimeSeries::range_iter`]
//!   for lazy ranges.
//! - Use [`AnyTime`] when a collection may contain values from multiple scales.
//! - Enable the `serde` feature to serialize and deserialize [`Time`],
//!   [`AnyTime`], [`TimeSeries`], and [`TimeScale`] values.
//!
// Astronomical time-scale names intentionally retain their standard acronyms.
#![allow(clippy::upper_case_acronyms)]
#![deny(missing_docs)]

mod anytime;
mod constants;
mod eop;
mod macros;
#[cfg(test)]
mod references;
pub mod scales;
mod series;
mod time;

pub use anytime::{AnyTime, AnyTimeVec};
pub use eop::{load_finals2000a, FinalsLoadError};
pub use scales::{
    TimeScale, TimeScaleParseError, GPS, GPST, TAI, TCB, TCG, TDB, TDT, TT, UT, UT1, UTC,
};
pub use series::{TimeSeries, TimeSeriesRange};
pub use time::{Scale, Time};

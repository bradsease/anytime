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
//! let tai: Time<TAI> = utc.clone().into();
//! assert_eq!(tai.utc(), utc);
//! ```
//!
//! The crate supports UTC, TAI, GPST, TT, TCG, and UT1. UTC conversions use
//! leap-second data built into the crate. UT1 conversions additionally use
//! Earth-orientation data loaded with [`load_finals2000a`].
//!
//! # Feature overview
//!
//! - Use [`Time::from_jd`], [`Time::from_mjd`], or [`Time::from_gregorian`]
//!   to construct a value.
//! - Use [`Time::utc`], [`Time::tai`], and the other scale methods to convert
//!   values.
//! - Use [`TimeSeries`] for stored collections or [`TimeSeries::range_iter`]
//!   for lazy ranges.
//! - Use [`AnyTime`] when a collection may contain values from multiple scales.
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
pub use scales::{GPS, GPST, TAI, TCG, TDT, TT, UT, UT1, UTC};
pub use series::{TimeSeries, TimeSeriesRange};
pub use time::{Scale, Time};

#![allow(unused_imports)]
#![allow(dead_code)]

//! Marker types and runtime selection for the time scales supported by [`crate::Time`].
//!
//! The main marker names are [`TAI`], [`TCB`], [`TCG`], [`TDB`], [`TT`], [`UT1`],
//! and [`UTC`]. The GNSS marker names are [`BDT`], [`GLONASST`], [`GPST`], [`GST`],
//! and [`QZZST`]. [`GPS`], [`TDT`], and [`UT`] are aliases for the corresponding
//! standard names.

mod bdt;
pub(crate) mod common;
mod glonasst;
mod gpst;
mod gst;
mod qzzst;
mod tai;
mod tcb;
mod tcg;
mod tdb;
mod tt;
mod ut1;
mod utc;

pub use bdt::BDT;
pub use glonasst::GLONASST;
pub use gpst::GPST;
pub use gst::GST;
pub use qzzst::QZZST;
pub use tai::TAI;
pub use tcb::TCB;
pub use tcg::TCG;
pub use tdb::TDB;
pub use tt::TT;
pub use ut1::UT1;
pub use utc::UTC;

use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// A time scale selected at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeScale {
    /// BeiDou Time.
    BDT,
    /// GLONASS Time.
    GLONASST,
    /// GPS Time.
    GPST,
    /// Galileo System Time.
    GST,
    /// QZSS Time.
    QZZST,
    /// International Atomic Time.
    TAI,
    /// Barycentric Coordinate Time.
    TCB,
    /// Geocentric Coordinate Time.
    TCG,
    /// Barycentric Dynamical Time.
    TDB,
    /// Terrestrial Time.
    TT,
    /// Universal Time 1.
    UT1,
    /// Coordinated Universal Time.
    UTC,
}

/// An error returned when a time-scale name is not supported.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeScaleParseError;

impl fmt::Display for TimeScaleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("unsupported time scale")
    }
}

impl Error for TimeScaleParseError {}

impl fmt::Display for TimeScale {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for TimeScale {
    type Err = TimeScaleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TimeScale {
    /// Returns the standard acronym for this scale.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BDT => "BDT",
            Self::GLONASST => "GLONASST",
            Self::GPST => "GPST",
            Self::GST => "GST",
            Self::QZZST => "QZZST",
            Self::TAI => "TAI",
            Self::TCB => "TCB",
            Self::TCG => "TCG",
            Self::TDB => "TDB",
            Self::TT => "TT",
            Self::UT1 => "UT1",
            Self::UTC => "UTC",
        }
    }

    /// Selects a scale from its standard acronym, ignoring ASCII case.
    ///
    /// The aliases `GPS`, `TDT`, and `UT` select [`Self::GPST`], [`Self::TT`],
    /// and [`Self::UT1`], respectively.
    pub fn parse(value: &str) -> Result<Self, TimeScaleParseError> {
        let value = value.trim();

        if value.eq_ignore_ascii_case("BDT") {
            Ok(Self::BDT)
        } else if value.eq_ignore_ascii_case("GLONASST") {
            Ok(Self::GLONASST)
        } else if value.eq_ignore_ascii_case("GPST") || value.eq_ignore_ascii_case("GPS") {
            Ok(Self::GPST)
        } else if value.eq_ignore_ascii_case("GST") {
            Ok(Self::GST)
        } else if value.eq_ignore_ascii_case("QZZST") {
            Ok(Self::QZZST)
        } else if value.eq_ignore_ascii_case("TAI") {
            Ok(Self::TAI)
        } else if value.eq_ignore_ascii_case("TCB") {
            Ok(Self::TCB)
        } else if value.eq_ignore_ascii_case("TCG") {
            Ok(Self::TCG)
        } else if value.eq_ignore_ascii_case("TDB") {
            Ok(Self::TDB)
        } else if value.eq_ignore_ascii_case("TT") || value.eq_ignore_ascii_case("TDT") {
            Ok(Self::TT)
        } else if value.eq_ignore_ascii_case("UT1") || value.eq_ignore_ascii_case("UT") {
            Ok(Self::UT1)
        } else if value.eq_ignore_ascii_case("UTC") {
            Ok(Self::UTC)
        } else {
            Err(TimeScaleParseError)
        }
    }
}

/// Alias for [`TT`], formerly called Terrestrial Dynamical Time.
pub type TDT = TT;

/// Alias for [`GPST`].
pub type GPS = GPST;

/// Alias for [`UT1`].
pub type UT = UT1;

#[cfg(test)]
mod tests {
    use super::TimeScale;

    #[test]
    fn parses_time_scale_names_and_aliases() {
        assert_eq!(TimeScale::parse("utc"), Ok(TimeScale::UTC));
        assert_eq!(TimeScale::parse("bdt"), Ok(TimeScale::BDT));
        assert_eq!(TimeScale::parse("glonasst"), Ok(TimeScale::GLONASST));
        assert_eq!("GPS".parse(), Ok(TimeScale::GPST));
        assert_eq!(TimeScale::parse("gst"), Ok(TimeScale::GST));
        assert_eq!(TimeScale::parse("qzzst"), Ok(TimeScale::QZZST));
        assert_eq!(TimeScale::parse("TDT"), Ok(TimeScale::TT));
        assert_eq!(TimeScale::parse("UT"), Ok(TimeScale::UT1));
        assert!(TimeScale::parse("invalid").is_err());
    }

    #[test]
    fn formats_and_parses_standard_time_scale_names() {
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
            assert_eq!(scale.to_string(), scale.as_str());
            assert_eq!(scale.as_str().parse(), Ok(scale));
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_round_trip() {
        let json = serde_json::to_string(&TimeScale::UTC).unwrap();

        assert_eq!(
            serde_json::from_str::<TimeScale>(&json).unwrap(),
            TimeScale::UTC
        );
    }
}

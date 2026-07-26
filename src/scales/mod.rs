#![allow(unused_imports)]
#![allow(dead_code)]

//! Marker types for the time scales supported by [`crate::Time`].
//!
//! The long-form marker names are [`GPST`], [`TAI`], [`TCG`], [`TT`],
//! [`UT1`], and [`UTC`]. [`GPS`], [`TDT`], and [`UT`] are aliases for the
//! corresponding standard names.

pub(crate) mod common;
mod gpst;
mod tai;
mod tcg;
mod tt;
mod ut1;
mod utc;

pub use gpst::GPST;
pub use tai::TAI;
pub use tcg::TCG;
pub use tt::TT;
pub use ut1::UT1;
pub use utc::UTC;

/// Alias for [`TT`], formerly called Terrestrial Dynamical Time.
pub type TDT = TT;

/// Alias for [`GPST`].
pub type GPS = GPST;

/// Alias for [`UT1`].
pub type UT = UT1;

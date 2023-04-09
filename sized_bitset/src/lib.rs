//! # sized_bitset
//!
//! This library provides `SizedBitset` (statically-sized bitset) and functionality for its.
//!
//! ## SizedBitset 101
//!
//! - constant initialization
//!
//! ```
//! use sized_bitset::SizedBitset;
//! const BITSET: SizedBitset<4> = SizedBitset::from_const([true, true, false, false]);
//! ```
//!
//! - from primitives
//!
//! `From<u{N}> SizedBitset<{M}>` is defined if and only if `N <= M`.
//!
//! ```
//! use sized_bitset::SizedBitset;
//! let bitset: SizedBitset<8> = 0b10101010.into();
//! ```
//!
//! - try from slice
//!
//! This panics if slice length is not exactly same as `SizedBitset` size.
//!
//! ```
//!
//! use sized_bitset::SizedBitset;
//! let bitset: SizedBitset<4> = [true, true, true, true].as_slice().try_into().unwrap();
//! ```
//!
//! - parse str
//!
//! This panics if `str` length is greater than `SizedBitset` size.
//!
//! ```
//!
//! use sized_bitset::SizedBitset;
//! let bitset: SizedBitset<4> = "1010".parse().unwrap();
//! ```
//!
//! ## To Primitives
//!
//! Allows `sized_bitset::convert::*;` us to convert [SizedBitset] to primitives.
//!
//! `To{N} for SizedBitset<{M}>` is defined if and only if `N >= M`.
//!
//! ```
//! pub mod convert {
//!     pub trait To8 {
//!         fn to_u8(&self) -> u8;
//!     }
//!     pub trait To16 {
//!         fn to_u16(&self) -> u16;
//!     }
//!     pub trait To32 {
//!         fn to_u32(&self) -> u32;
//!     }
//!     pub trait To64 {
//!         fn to_u64(&self) -> u64;
//!     }
//!     pub trait To128 {
//!         fn to_u128(&self) -> u128;
//!     }
//! }
//! ```
//!

mod bitset;
pub mod error;

pub use bitset::*;

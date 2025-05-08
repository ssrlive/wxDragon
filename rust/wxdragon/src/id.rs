//! Module for handling wxWidgets IDs.

use wxdragon_sys as ffi;

// Define Id as a type alias for i32
pub type Id = i32;

// Re-export the WXD_ID_ANY constant from ffi
pub use ffi::WXD_ID_ANY;

/// The highest standard ID.
/// Used as a base for custom IDs (e.g., `ID_HIGHEST + 1`).
pub const ID_HIGHEST: Id = ffi::WXD_ID_HIGHEST as Id;

/// Standard ID for Exit commands.
pub const ID_EXIT: Id = ffi::WXD_ID_EXIT as Id;

/// Standard ID for About commands.
pub const ID_ABOUT: Id = ffi::WXD_ID_ABOUT as Id;

// ID_NONE is not typically exported/used directly this way;
// comparisons usually happen against ID_ANY or specific IDs.
// pub const ID_NONE: Id = ffi::WXD_ID_NONE; // WXD_ID_NONE wasn't generated

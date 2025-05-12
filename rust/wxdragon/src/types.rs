//! Common type definitions and traits.
//!
//! This module contains fundamental type aliases, constants, and traits
//! that are used throughout the wxDragon library.

/// Style flags type, matching wxWidgets' wxd_Style_t.
/// 
/// Used for widget and window style flags.
pub type Style = i64;

/// Standard identifier type for widgets.
/// 
/// Matches wxWidgets int-based ID system.
pub type Id = i32;

/// Matches wxID_ANY (-1), used for auto-generated IDs.
pub const ID_ANY: Id = -1;

/// Trait to get the raw FFI pointer of a specific widget type.
/// 
/// This trait is implemented by widgets to provide access to
/// their underlying wxWidgets native pointers.
pub trait RawWxProps {
    /// Associated type for the specific FFI pointer (e.g., wxd_Button_t)
    type RawWxPtr;
    
    /// Returns the raw FFI pointer to the underlying wxWidgets object
    fn raw_wx_ptr(&self) -> *mut Self::RawWxPtr;
} 
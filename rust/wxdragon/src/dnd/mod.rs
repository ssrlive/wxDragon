//! Drag and drop functionality for wxDragon applications.
//!
//! This module provides classes for implementing drag and drop operations
//! in wxDragon applications, following the wxWidgets drag and drop pattern.

mod dataobject;
mod dropsource;
mod droptarget;

pub use dataobject::*;
pub use dropsource::*;
pub use droptarget::*;

use std::fmt;

/// Represents the result of a drag and drop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragResult {
    /// Nothing happened (drag target didn't accept data).
    None,
    /// The data was copied.
    Copy,
    /// The data was moved (source should delete it).
    Move,
    /// A link to the data was created.
    Link,
    /// The operation was cancelled by the user.
    Cancel,
    /// An error occurred during drag and drop.
    Error,
}

impl DragResult {
    /// Creates a `DragResult` from the C enum value.
    pub(crate) fn from_c_enum(result: wxdragon_sys::WXDDragResultCEnum) -> Self {
        match result {
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_NONE => DragResult::None,
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_COPY => DragResult::Copy,
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_MOVE => DragResult::Move,
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_LINK => DragResult::Link,
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_CANCEL => DragResult::Cancel,
            wxdragon_sys::WXDDragResultCEnum_WXD_DRAG_ERROR | _ => DragResult::Error,
        }
    }
}

impl fmt::Display for DragResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DragResult::None => write!(f, "None"),
            DragResult::Copy => write!(f, "Copy"),
            DragResult::Move => write!(f, "Move"),
            DragResult::Link => write!(f, "Link"),
            DragResult::Cancel => write!(f, "Cancel"),
            DragResult::Error => write!(f, "Error"),
        }
    }
} 
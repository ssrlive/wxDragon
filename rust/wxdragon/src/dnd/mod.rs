//! Drag and drop functionality for wxDragon applications.
//!
//! This module provides classes for implementing drag and drop operations
//! in wxDragon applications, following the wxWidgets drag and drop pattern.

pub mod dataobject;
pub mod dropsource;
pub mod droptarget;

pub use dataobject::{DataObject, FileDataObject, TextDataObject};
pub use dropsource::DropSource;
pub use droptarget::{FileDropTarget, TextDropTarget};

use std::fmt;

/// The result of a drag and drop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DragResult {
    /// No drag operation.
    None = 0,
    /// Copy the data.
    Copy = 1,
    /// Move the data.
    Move = 2,
    /// Link to the data.
    Link = 3,
    /// Cancel the drag.
    Cancel = 4,
    /// Error in drag operation.
    Error = 5,
}

impl Default for DragResult {
    fn default() -> Self {
        DragResult::None
    }
}

impl From<i32> for DragResult {
    fn from(result: i32) -> Self {
        match result {
            0 => DragResult::None,
            1 => DragResult::Copy,
            2 => DragResult::Move,
            3 => DragResult::Link,
            4 => DragResult::Cancel,
            5 => DragResult::Error,
            _ => DragResult::None,
        }
    }
}

impl From<DragResult> for i32 {
    fn from(result: DragResult) -> Self {
        result as i32
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

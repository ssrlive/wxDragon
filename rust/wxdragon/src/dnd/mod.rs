//! Drag and drop functionality for wxDragon applications.
//!
//! This module provides classes for implementing drag and drop operations
//! in wxDragon applications, following the wxWidgets drag and drop pattern.

mod dropsource;
mod droptarget;
// Use the main data_object module instead of our own implementation
// mod dataobject;

pub use dropsource::DropSource;
pub use droptarget::{FileDropTarget, TextDropTarget};
// Re-export data objects from the main module
pub use crate::data_object::{DataObject, FileDataObject, TextDataObject, BitmapDataObject};

use std::fmt;

/// The result of a drag and drop operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DragResult {
    /// No effect (drag target didn't accept the data).
    None = 0,
    
    /// The data was copied.
    Copy = 1,
    
    /// The data was moved (ownership transferred).
    Move = 2,
    
    /// Link to the data.
    Link = 3,
    
    /// The drag operation was canceled by the user.
    Cancel = 4,
    
    /// Error in the drag operation.
    Error = 5,
}

impl From<i32> for DragResult {
    fn from(value: i32) -> Self {
        match value {
            0 => DragResult::None,
            1 => DragResult::Copy,
            2 => DragResult::Move,
            3 => DragResult::Link,
            4 => DragResult::Cancel,
            _ => DragResult::Error,
        }
    }
}

impl Into<i32> for DragResult {
    fn into(self) -> i32 {
        match self {
            DragResult::None => 0,
            DragResult::Copy => 1,
            DragResult::Move => 2,
            DragResult::Link => 3,
            DragResult::Cancel => 4,
            DragResult::Error => 5,
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

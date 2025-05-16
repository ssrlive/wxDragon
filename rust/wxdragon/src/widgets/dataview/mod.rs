//! DataView widget family implementation.
//!
//! This module provides Rust wrappers for wxWidget's DataView controls:
//! - `DataViewCtrl`: Base control for displaying data in a customizable view.
//! - `DataViewListCtrl`: Simplified list-based DataView control.
//! - `DataViewTreeCtrl`: Tree-based DataView control.
//!
//! It also provides supporting types for renderers, columns, and data models.
//!
//! # Important Notes
//! 
//! When using the DataView controls, remember these key points:
//!
//! 1. Define columns in the DataViewListModel *before* adding any rows
//! 2. Keep track of the DataViewListModel as long as the control exists
//! 3. The order of operations matters:
//!    - Create the DataViewCtrl
//!    - Add visual columns to the control
//!    - Create and configure the model
//!    - Associate the model with the control
//!    - Add data to the model

// Re-export all components from submodules
mod ctrl;
mod column;
mod renderer;
mod model;
mod variant;
mod list_ctrl;
mod tree_ctrl;

// Public exports
pub use ctrl::*;
pub use column::*;
pub use renderer::*;
pub use model::*;
pub use variant::*;
pub use list_ctrl::*;
pub use tree_ctrl::*; 
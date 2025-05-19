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

// Internal module structure
mod ctrl;
mod column;
mod renderer;
mod model;
mod variant;
mod list_ctrl;
mod tree_ctrl;
mod enums;
mod item;

// Public exports
pub use ctrl::{DataViewCtrl, DataViewCtrlBuilder, DataViewStyle};
pub use list_ctrl::{DataViewListCtrl, DataViewListCtrlBuilder};
pub use tree_ctrl::{DataViewTreeCtrl, DataViewTreeCtrlBuilder};
pub use column::DataViewColumn;
pub use model::{
    DataViewModel, 
    DataViewListModel,
    DataViewVirtualListModel,
    CustomDataViewVirtualListModel,
    DataViewItemAttr,
};
pub use variant::{VariantType, Variant};
pub use enums::{DataViewCellMode, DataViewAlign};
pub use item::DataViewItem;

// Re-export all renderers
pub use renderer::{
    DataViewRenderer,
    DataViewTextRenderer,
    DataViewToggleRenderer, 
    DataViewProgressRenderer,
    DataViewIconTextRenderer,
    DataViewBitmapRenderer,
    DataViewDateRenderer,
    DataViewSpinRenderer,
    DataViewChoiceRenderer,
    DataViewCheckIconTextRenderer,
}; 
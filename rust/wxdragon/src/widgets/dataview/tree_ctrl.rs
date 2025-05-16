//! DataViewTreeCtrl implementation.

use crate::{Id, Point, Size, Window, WxWidget, WxEvtHandler};
use crate::{widget_builder, implement_widget_traits_with_target};
use wxdragon_sys as ffi;

use super::DataViewStyle;

/// A specialized DataViewCtrl that displays data in a tree format.
///
/// DataViewTreeCtrl is a convenience wrapper around DataViewCtrl that simplifies
/// the display of hierarchical data in a tree structure.
pub struct DataViewTreeCtrl {
    window: Window,
}

impl DataViewTreeCtrl {
    /// Creates a builder for configuring and constructing a DataViewTreeCtrl.
    pub fn builder(parent: &dyn WxWidget) -> DataViewTreeCtrlBuilder {
        DataViewTreeCtrlBuilder::new(parent)
    }

    fn new_impl(parent_ptr: *mut ffi::wxd_Window_t, id: i32, pos: Point,
               size: Size, style: i64) -> Self {
        let handle = unsafe {
            ffi::wxd_DataViewTreeCtrl_Create(
                parent_ptr,
                id as i64,
                &pos as *const Point as *const ffi::wxd_Point,
                &size as *const Size as *const ffi::wxd_Size,
                style,
            )
        };

        let window = unsafe { Window::from_ptr(handle) };
        Self { window }
    }
}

implement_widget_traits_with_target!(DataViewTreeCtrl, window, Window);

widget_builder!(
    name: DataViewTreeCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DataViewStyle,
    fields: {},
    build_impl: |slf| {
        DataViewTreeCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
); 
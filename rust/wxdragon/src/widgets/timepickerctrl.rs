use std::ptr;

use wxdragon_sys as ffi;

use crate::datetime::DateTime;
use crate::event::WxEvtHandler;
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::default::Default;

// --- Style enum using macro ---
widget_style_enum!(
    name: TimePickerCtrlStyle,
    doc: "Style flags for TimePickerCtrl widgets.",
    variants: {
        Default: 0, "Default style."
    },
    default_variant: Default
);

// --- wxTimePickerCtrl ---
#[derive(Clone)]
pub struct TimePickerCtrl {
    window: Window, // Embed Window for common wxWindow functionality
}

impl TimePickerCtrl {
    /// Creates a new TimePickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> TimePickerCtrlBuilder {
        TimePickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected time.
    pub fn get_value(&self) -> DateTime {
        let ffi_dt = unsafe {
            ffi::wxd_TimePickerCtrl_GetValue(self.window.as_ptr() as *mut ffi::wxd_TimePickerCtrl_t)
        };
        DateTime::from_raw(ffi_dt)
    }

    /// Sets the currently selected time.
    pub fn set_value(&self, dt: &DateTime) {
        unsafe {
            ffi::wxd_TimePickerCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_TimePickerCtrl_t,
                dt.as_ptr(),
            );
        }
    }

    /// Creates a TimePickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_TimePickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_TimePickerCtrl_t) -> Self {
        TimePickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Use the widget_builder macro to generate the TimePickerCtrlBuilder implementation
widget_builder!(
    name: TimePickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: TimePickerCtrlStyle,
    fields: {
        value: Option<DateTime> = None
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "TimePickerCtrl requires a parent");

        let ffi_dt_ptr = slf.value.as_ref().map_or(ptr::null(), |dt_val| dt_val.as_ptr());

        let ptr = unsafe {
            ffi::wxd_TimePickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                ffi_dt_ptr,
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits(),
            )
        };
        if ptr.is_null() {
            panic!("Failed to create TimePickerCtrl: FFI returned null pointer.");
        } else {
            unsafe { TimePickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits macro to implement traits
implement_widget_traits_with_target!(TimePickerCtrl, window, Window); 
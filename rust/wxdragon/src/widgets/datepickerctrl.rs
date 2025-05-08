use std::ptr;

use wxdragon_sys as ffi;

use crate::event::WxEvtHandler;
use crate::prelude::*;
use crate::window::{Window, WxWidget};
use std::default::Default;

// --- Constants for wxDatePickerCtrl ---
// Styles from wx/datectrl.h (via wxdragon.h)
pub const DP_SPIN: i64 = ffi::WXD_DP_SPIN;
pub const DP_DROPDOWN: i64 = ffi::WXD_DP_DROPDOWN;
pub const DP_DEFAULT: i64 = ffi::WXD_DP_DEFAULT;
pub const DP_ALLOWNONE: i64 = ffi::WXD_DP_ALLOWNONE;
pub const DP_SHOWCENTURY: i64 = ffi::WXD_DP_SHOWCENTURY;

// Event type
pub const EVT_DATE_CHANGED: u32 = ffi::WXDEventTypeCEnum_WXD_EVENT_TYPE_DATE_CHANGED;

// --- DateTime struct ---
// This mirrors wxd_DateTime_t from wxdragon.h
// Note: wxDateTime month is 0-indexed (January = 0), which matches our C struct and this Rust struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub day: i16,    // 1-31
    pub month: u16,  // 0-11 (0 = January, 11 = December)
    pub year: i32,   // e.g., 2023
    pub hour: i16,   // 0-23
    pub minute: i16, // 0-59
    pub second: i16, // 0-59
}

impl DateTime {
    /// Creates a new DateTime.
    /// Month is 0-indexed (0 for January, 11 for December).
    pub fn new(year: i32, month: u16, day: i16, hour: i16, minute: i16, second: i16) -> Self {
        DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Gets the current date and time.
    pub fn now() -> Self {
        let ffi_dt = unsafe { ffi::wxd_DateTime_Now() };
        Self::from_ffi_date_time(&ffi_dt)
    }

    /// Returns a default, often invalid, DateTime.
    /// Use `is_valid()` to check.
    pub fn default_value() -> Self {
        let ffi_dt = unsafe { ffi::wxd_DateTime_Default() };
        Self::from_ffi_date_time(&ffi_dt)
    }

    /// Checks if the DateTime is valid.
    pub fn is_valid(&self) -> bool {
        let ffi_dt = self.to_ffi_date_time();
        unsafe { ffi::wxd_DateTime_IsValid(&ffi_dt) }
    }

    pub(crate) fn to_ffi_date_time(&self) -> ffi::wxd_DateTime_t {
        ffi::wxd_DateTime_t {
            day: self.day,
            month: self.month,
            year: self.year,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        }
    }

    pub(crate) fn from_ffi_date_time(ffi_dt: &ffi::wxd_DateTime_t) -> Self {
        DateTime {
            day: ffi_dt.day,
            month: ffi_dt.month,
            year: ffi_dt.year,
            hour: ffi_dt.hour,
            minute: ffi_dt.minute,
            second: ffi_dt.second,
        }
    }
}

// --- wxDatePickerCtrl ---
#[derive(Clone)]
pub struct DatePickerCtrl {
    window: Window, // Embed Window for common wxWindow functionality
}

impl DatePickerCtrl {
    /// Creates a new DatePickerCtrlBuilder.
    pub fn builder(parent: &impl WxWidget) -> DatePickerCtrlBuilder {
        let mut builder = DatePickerCtrlBuilder::default();
        builder.parent_ptr = parent.handle_ptr();
        builder
    }

    /// Gets the currently selected date.
    pub fn get_value(&self) -> DateTime {
        let ffi_dt = unsafe {
            ffi::wxd_DatePickerCtrl_GetValue(self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t)
        };
        DateTime::from_ffi_date_time(&ffi_dt)
    }

    /// Sets the currently selected date.
    pub fn set_value(&self, dt: &DateTime) {
        let ffi_dt = dt.to_ffi_date_time();
        unsafe {
            ffi::wxd_DatePickerCtrl_SetValue(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                &ffi_dt,
            );
        }
    }

    /// Gets the valid range for dates on the control.
    /// Returns `Ok((Option<DateTime>, Option<DateTime>))` if successful.
    /// The DateTimes in the tuple will be None if the corresponding bound is not set or if the bounds are invalid.
    pub fn get_range(&self) -> Result<(Option<DateTime>, Option<DateTime>), String> {
        let mut ffi_dt1 = DateTime::default_value().to_ffi_date_time();
        let mut ffi_dt2 = DateTime::default_value().to_ffi_date_time();

        // GetRange C FFI returns true if a range is set, false otherwise.
        // It populates dt1 and dt2 regardless; they will be invalid if no range or only one side of range is set.
        let _has_range = unsafe {
            ffi::wxd_DatePickerCtrl_GetRange(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                &mut ffi_dt1,
                &mut ffi_dt2,
            )
        };

        let dt1 = DateTime::from_ffi_date_time(&ffi_dt1);
        let dt2 = DateTime::from_ffi_date_time(&ffi_dt2);

        let opt_dt1 = if dt1.is_valid() { Some(dt1) } else { None };
        let opt_dt2 = if dt2.is_valid() { Some(dt2) } else { None };

        Ok((opt_dt1, opt_dt2))
    }

    /// Sets the valid range for dates on the control.
    /// Pass `None` for `dt_start` or `dt_end` to remove the lower or upper bound, respectively.
    pub fn set_range(&self, dt_start: Option<&DateTime>, dt_end: Option<&DateTime>) {
        let ffi_dt1_stack: Option<ffi::wxd_DateTime_t> = dt_start.map(|dt| dt.to_ffi_date_time());
        let ffi_dt2_stack: Option<ffi::wxd_DateTime_t> = dt_end.map(|dt| dt.to_ffi_date_time());

        let ptr_dt1 = ffi_dt1_stack
            .as_ref()
            .map_or(ptr::null(), |val| val as *const _);
        let ptr_dt2 = ffi_dt2_stack
            .as_ref()
            .map_or(ptr::null(), |val| val as *const _);

        unsafe {
            ffi::wxd_DatePickerCtrl_SetRange(
                self.window.as_ptr() as *mut ffi::wxd_DatePickerCtrl_t,
                ptr_dt1,
                ptr_dt2,
            );
        }
    }

    /// Creates a DatePickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_DatePickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_DatePickerCtrl_t) -> Self {
        DatePickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

impl WxWidget for DatePickerCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl WxEvtHandler for DatePickerCtrl {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// --- DatePickerCtrlBuilder ---
pub struct DatePickerCtrlBuilder {
    parent_ptr: *mut ffi::wxd_Window_t,
    id: Id,
    dt: Option<DateTime>,
    pos: Point,
    size: Size,
    style: i64,
}

impl Default for DatePickerCtrlBuilder {
    fn default() -> Self {
        Self {
            parent_ptr: ptr::null_mut(),
            id: -1,
            dt: None,
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            style: DP_DEFAULT,
        }
    }
}

impl DatePickerCtrlBuilder {
    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_value(mut self, dt: DateTime) -> Self {
        self.dt = Some(dt);
        self
    }

    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> DatePickerCtrl {
        assert!(
            !self.parent_ptr.is_null(),
            "DatePickerCtrl requires a parent"
        );

        let ffi_dt_on_stack: Option<ffi::wxd_DateTime_t> =
            self.dt.as_ref().map(|dt_val| dt_val.to_ffi_date_time());
        let ffi_dt_ptr = ffi_dt_on_stack
            .as_ref()
            .map_or(ptr::null(), |dt_ref| dt_ref as *const _);

        let ffi_pos: ffi::wxd_Point = self.pos.into();
        let ffi_size: ffi::wxd_Size = self.size.into();

        let ptr = unsafe {
            ffi::wxd_DatePickerCtrl_Create(
                self.parent_ptr,
                self.id,
                ffi_dt_ptr,
                ffi_pos,
                ffi_size,
                self.style as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create DatePickerCtrl: FFI returned null pointer.");
        } else {
            unsafe { DatePickerCtrl::from_ptr(ptr) }
        }
    }
}

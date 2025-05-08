use crate::datetime::DateTime;
use crate::prelude::*;
use crate::window::{Window, WxWidget};
use std::ops::{Deref, Drop};
use wxdragon_sys as ffi;

// CalendarCtrl specific style constants can be added here if exposed via WXD_*
// Example:
// pub const CAL_SUNDAY_FIRST: i64 = ffi::WXD_CAL_SUNDAY_FIRST;
// pub const CAL_SHOW_HOLIDAYS: i64 = ffi::WXD_CAL_SHOW_HOLIDAYS;
// pub const CAL_NO_YEAR_CHANGE: i64 = ffi::WXD_CAL_NO_YEAR_CHANGE;
// pub const CAL_NO_MONTH_CHANGE: i64 = ffi::WXD_CAL_NO_MONTH_CHANGE;
// pub const CAL_SEQUENTIAL_MONTH_SELECTION: i64 = ffi::WXD_CAL_SEQUENTIAL_MONTH_SELECTION;
// pub const CAL_SHOW_SURROUNDING_WEEKS: i64 = ffi::WXD_CAL_SHOW_SURROUNDING_WEEKS;

/// Represents a `wxCalendarCtrl`.
#[derive(Clone)]
pub struct CalendarCtrl {
    window: Window,
}

impl CalendarCtrl {
    /// Creates a new `CalendarCtrlBuilder` for constructing a calendar control.
    pub fn builder(parent: &dyn WxWidget) -> CalendarCtrlBuilder {
        CalendarCtrlBuilder::new(parent)
    }

    /// Low-level constructor used by the builder.
    fn new(
        parent: &dyn WxWidget,
        id: Id,
        date: Option<&DateTime>, // Use Option<&DateTime> to pass to FFI
        pos: Point,
        size: Size,
        style: i64,
    ) -> Option<Self> {
        // Convert Option<&DateTime> to *const ffi::wxd_DateTime_t
        // If date is None, pass null. If Some, pass pointer to its raw ffi::wxd_DateTime_t.
        let c_date_ptr: *const ffi::wxd_DateTime_t = date.map_or(std::ptr::null(), |d| d.as_ptr());

        unsafe {
            let parent_ptr = parent.handle_ptr();
            if parent_ptr.is_null() {
                return None;
            }
            let ctrl_ptr = ffi::wxd_CalendarCtrl_Create(
                parent_ptr as *mut _,
                id,
                c_date_ptr, // Pass the potentially null pointer
                pos.into(),
                size.into(),
                style.try_into().unwrap_or(0), // Default style to 0 if conversion fails
            );
            if ctrl_ptr.is_null() {
                None
            } else {
                let window = Window::from_ptr(ctrl_ptr as *mut ffi::wxd_Window_t);
                Some(CalendarCtrl { window })
            }
        }
    }

    /// Sets the currently displayed date.
    pub fn set_date(&self, date: &DateTime) -> bool {
        unsafe { ffi::wxd_CalendarCtrl_SetDate(self.window.as_ptr() as *mut _, date.as_ptr()) }
    }

    /// Gets the currently displayed date.
    pub fn get_date(&self) -> DateTime {
        unsafe {
            let raw_dt = ffi::wxd_CalendarCtrl_GetDate(self.window.as_ptr() as *mut _);
            DateTime::from_raw(raw_dt)
        }
    }
}

// --- CalendarCtrl Builder ---

/// Builder pattern for creating `CalendarCtrl` widgets.
#[derive(Clone)] // DateTime is Copy, so builder can be Clone
pub struct CalendarCtrlBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    initial_date: Option<DateTime>, // Store an owned DateTime for the builder
    pos: Point,
    size: Size,
    style: i64,
}

impl<'a> CalendarCtrlBuilder<'a> {
    /// Creates a new builder.
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: crate::id::ID_ANY as Id, // Corrected usage
            initial_date: None,
            pos: Point { x: -1, y: -1 }, // Standard default
            size: Size {
                width: -1,
                height: -1,
            }, // Standard default
            style: 0,
        }
    }

    /// Sets the window identifier.
    pub fn with_id(mut self, id: Id) -> Self {
        // Ensure id is of type Id
        self.id = id;
        self
    }

    /// Sets the initially displayed date.
    /// If not set, the control defaults to today's date.
    pub fn with_initial_date(mut self, date: DateTime) -> Self {
        self.initial_date = Some(date);
        self
    }

    /// Sets the position.
    pub fn with_pos(mut self, pos: Point) -> Self {
        self.pos = pos;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets the window style flags.
    /// Common styles include `CAL_SUNDAY_FIRST`, `CAL_SHOW_HOLIDAYS`.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Builds the `CalendarCtrl`.
    pub fn build(self) -> CalendarCtrl {
        CalendarCtrl::new(
            self.parent,
            self.id,
            self.initial_date.as_ref(), // Pass Option<&DateTime>
            self.pos,
            self.size,
            self.style,
        )
        .expect("Failed to create CalendarCtrl widget")
    }
}

// Implement WxWidget trait
impl WxWidget for CalendarCtrl {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

// Implement Drop
impl Drop for CalendarCtrl {
    fn drop(&mut self) {
        // No explicit destruction needed here. The `Window` contained within
        // (if it had specific drop logic) would run, but wxWidgets typically
        // handles child widget destruction when the parent is destroyed.
        // If specific cleanup were needed for CalendarCtrl beyond what Window provides,
        // it would go here (e.g., calling a specific ffi::wxd_CalendarCtrl_Destroy if it existed
        // and was necessary, but this is not the wxWidgets pattern for child controls).
    }
}

// Allow CalendarCtrl to be used where a Window is expected (e.g., for sizers)
impl Deref for CalendarCtrl {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}

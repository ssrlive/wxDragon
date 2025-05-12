use crate::geometry::{Point, Size};
use crate::datetime::DateTime;
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::implement_widget_traits_with_target;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ptr;
use wxdragon_sys as ffi;

// Define a proper style enum for CalendarCtrl
widget_style_enum!(
    name: CalendarCtrlStyle,
    doc: "Style flags for Calendar control.",
    variants: {
        Default: 0, "Default style.",
        SundayFirst: ffi::WXD_CAL_SUNDAY_FIRST, "Show Sunday as the first day in the week.",
        MondayFirst: ffi::WXD_CAL_MONDAY_FIRST, "Show Monday as the first day in the week.",
        ShowHolidays: ffi::WXD_CAL_SHOW_HOLIDAYS, "Highlight holidays in the calendar.",
        NoYearChange: ffi::WXD_CAL_NO_YEAR_CHANGE, "Disable year changing.",
        NoMonthChange: ffi::WXD_CAL_NO_MONTH_CHANGE, "Disable month changing.",
        SequentialMonthSelection: ffi::WXD_CAL_SEQUENTIAL_MONTH_SELECTION, "Use alternative, more compact, style for the month and year selection controls.",
        ShowSurroundingWeeks: ffi::WXD_CAL_SHOW_SURROUNDING_WEEKS, "Show the neighbouring weeks in the previous and next months."
    },
    default_variant: Default
);

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
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        date: Option<&DateTime>,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "CalendarCtrl requires a parent");
        
        // Convert Option<&DateTime> to *const ffi::wxd_DateTime_t
        let c_date_ptr: *const ffi::wxd_DateTime_t = date.map_or(ptr::null(), |d| d.as_ptr());

        let ptr = unsafe {
            ffi::wxd_CalendarCtrl_Create(
                parent_ptr,
                id,
                c_date_ptr,
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };
        
        if ptr.is_null() {
            panic!("Failed to create CalendarCtrl widget");
        }
        
        unsafe {
            let window = Window::from_ptr(ptr as *mut ffi::wxd_Window_t);
            CalendarCtrl { window }
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

// Use the widget_builder macro for CalendarCtrl
widget_builder!(
    name: CalendarCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: CalendarCtrlStyle,
    fields: {
        initial_date: Option<DateTime> = None
    },
    build_impl: |slf| {
        CalendarCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.initial_date.as_ref(),
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Apply common trait implementations for this widget
implement_widget_traits_with_target!(CalendarCtrl, window, Window);

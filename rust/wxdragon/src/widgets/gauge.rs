use crate::geometry::{Point, Size, DEFAULT_POSITION, DEFAULT_SIZE};
use crate::event::WxEvtHandler;
use crate::id::{Id, ID_ANY};
use crate::window::{Window, WxWidget};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_int;
use wxdragon_sys as ffi;

// Opaque pointer type from FFI
pub type RawGauge = ffi::wxd_Gauge_t;

#[derive(Clone)]
pub struct Gauge {
    window: Window, // Embed the generic Window
}

impl Gauge {
    unsafe fn from_ptr(ptr: *mut RawGauge) -> Self {
        assert!(!ptr.is_null());
        Gauge {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    pub fn builder(parent: &impl WxWidget) -> GaugeBuilder {
        GaugeBuilder::new(parent)
    }

    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        range: i32,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        unsafe {
            let ctrl_ptr = ffi::wxd_Gauge_Create(
                parent_ptr,
                id,
                range as c_int,
                pos.x,
                pos.y,
                size.width,
                size.height,
                style as ffi::wxd_Style_t,
            );
            assert!(!ctrl_ptr.is_null(), "wxd_Gauge_Create returned null");
            Self::from_ptr(ctrl_ptr)
        }
    }

    pub fn set_range(&self, range: i32) {
        unsafe {
            ffi::wxd_Gauge_SetRange(self.window.handle_ptr() as *mut RawGauge, range as c_int)
        }
    }

    pub fn set_value(&self, value: i32) {
        unsafe {
            ffi::wxd_Gauge_SetValue(self.window.handle_ptr() as *mut RawGauge, value as c_int)
        }
    }

    pub fn get_value(&self) -> i32 {
        unsafe { ffi::wxd_Gauge_GetValue(self.window.handle_ptr() as *mut RawGauge) as i32 }
    }
}

// Delegate WxWidget and WxEvtHandler to the inner Window
impl WxWidget for Gauge {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.handle_ptr()
    }
}

impl WxEvtHandler for Gauge {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.get_event_handler_ptr()
    }
}

// Builder
pub struct GaugeBuilder<'a> {
    parent: &'a dyn WxWidget,
    id: Id,
    range: i32,
    pos: Option<Point>,
    size: Option<Size>,
    style: GaugeStyle,
}

impl<'a> GaugeBuilder<'a> {
    pub fn new(parent: &'a dyn WxWidget) -> Self {
        Self {
            parent,
            id: ID_ANY as Id,
            range: 100, // Default range
            pos: None,
            size: None,
            style: GaugeStyle::Default,
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_range(mut self, range: i32) -> Self {
        self.range = range;
        self
    }

    pub fn with_pos(mut self, x: i32, y: i32) -> Self {
        self.pos = Some(Point { x, y });
        self
    }

    pub fn with_size(mut self, width: i32, height: i32) -> Self {
        self.size = Some(Size { width, height });
        self
    }

    pub fn with_style(mut self, style: GaugeStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> Gauge {
        let parent_ptr = self.parent.handle_ptr();
        let pos = self.pos.unwrap_or(DEFAULT_POSITION);
        let size = self.size.unwrap_or(DEFAULT_SIZE);
        Gauge::new_impl(
            parent_ptr,
            self.id,
            self.range,
            pos,
            size,
            self.style.bits(),
        )
    }
}

// --- GaugeStyle Enum ---

/// Style flags for `Gauge`.
/// These flags can be combined using the bitwise OR operator (`|`).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum GaugeStyle {
    /// Default style (horizontal bar).
    Default = ffi::WXD_GA_HORIZONTAL,
    /// Vertical gauge.
    Vertical = ffi::WXD_GA_VERTICAL,
    /// Use smooth progress indication (typically native look and feel determines this).
    Smooth = ffi::WXD_GA_SMOOTH,
    /// Show textual progress (e.g., "50%").
    /// On some platforms, this might be the default or combined with non-smooth.
    ShowProgress = ffi::WXD_GA_PROGRESS,
}

impl GaugeStyle {
    /// Returns the raw integer value of the style.
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl BitOr for GaugeStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for GaugeStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        unsafe {
            *self = std::mem::transmute(self.bits() | rhs.bits());
        }
    }
}

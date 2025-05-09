use crate::base::{Point, Size, ID_ANY};
// use crate::defs::Style; // Removed unused import
use crate::event::WxEvtHandler;
use crate::id::Id;
use crate::window::{Window, WxWidget};
use std::ffi::{CStr, CString};
use std::ops::{BitOr, BitOrAssign};
use std::os::raw::c_char;
use std::ptr;
use wxdragon_sys as ffi;

// Constants from wxWidgets for RadioBox
// Values populated by const_extractor via ffi
// pub const RA_SPECIFY_COLS: i64 = ffi::WXD_RA_SPECIFY_COLS;
// pub const RA_SPECIFY_ROWS: i64 = ffi::WXD_RA_SPECIFY_ROWS;
// wxRB_GROUP, wxRB_SINGLE seem less relevant for RadioBox itself, more for RadioButton
// Default style includes wxRA_SPECIFY_COLS

/// Represents a wxRadioBox control.
#[derive(Clone)]
pub struct RadioBox {
    window: Window,
}

impl RadioBox {
    /// Creates a new `RadioBoxBuilder`.
    pub fn builder<'a>(
        parent: Option<&'a dyn WxWidget>,
        choices: &'a [&'a str],
    ) -> RadioBoxBuilder<'a> {
        RadioBoxBuilder::new(parent, choices)
    }

    /// Creates a `RadioBox` from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and represents a `wxRadioBox`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_RadioBox_t) -> Self {
        RadioBox {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }

    pub fn get_selection(&self) -> i32 {
        unsafe { ffi::wxd_RadioBox_GetSelection(self.as_ptr()) }
    }

    pub fn set_selection(&self, n: i32) {
        unsafe { ffi::wxd_RadioBox_SetSelection(self.as_ptr(), n) }
    }

    pub fn get_string(&self, n: i32) -> String {
        unsafe {
            let required_len_p1 = ffi::wxd_RadioBox_GetString(self.as_ptr(), n, ptr::null_mut(), 0);
            if required_len_p1 <= 0 {
                return String::new();
            }
            let capacity = required_len_p1 as usize;
            let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
            let success_code = ffi::wxd_RadioBox_GetString(
                self.as_ptr(),
                n,
                buffer.as_mut_ptr() as *mut c_char,
                capacity as i32,
            );

            if success_code == 0 {
                // Use CStr to find the length, including potential embedded nulls handled correctly.
                let c_str = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                String::from_utf8_lossy(c_str.to_bytes()).into_owned()
            } else {
                String::new() // Error occurred during copy
            }
        }
    }

    pub fn get_count(&self) -> u32 {
        unsafe { ffi::wxd_RadioBox_GetCount(self.as_ptr()) }
    }

    pub fn enable_item(&self, n: i32, enable: bool) -> bool {
        unsafe { ffi::wxd_RadioBox_EnableItem(self.as_ptr(), n, enable) }
    }

    pub fn is_item_enabled(&self, n: i32) -> bool {
        unsafe { ffi::wxd_RadioBox_IsItemEnabled(self.as_ptr(), n) }
    }

    pub fn show_item(&self, n: i32, show: bool) -> bool {
        unsafe { ffi::wxd_RadioBox_ShowItem(self.as_ptr(), n, show) }
    }

    pub fn is_item_shown(&self, n: i32) -> bool {
        unsafe { ffi::wxd_RadioBox_IsItemShown(self.as_ptr(), n) }
    }

    /// Returns the raw wxRadioBox pointer.
    fn as_ptr(&self) -> *mut ffi::wxd_RadioBox_t {
        self.window.as_ptr() as *mut _
    }
}

// --- Builder ---

pub struct RadioBoxBuilder<'a> {
    parent: Option<&'a dyn WxWidget>,
    id: Id,
    label: &'a str,
    choices: &'a [&'a str],
    pos: Point,
    size: Size,
    major_dimension: i32,
    style: RadioBoxStyle,
}

impl<'a> RadioBoxBuilder<'a> {
    pub fn new(parent: Option<&'a dyn WxWidget>, choices: &'a [&'a str]) -> Self {
        RadioBoxBuilder {
            parent,
            id: ID_ANY,
            label: "",
            choices,
            pos: Point { x: -1, y: -1 },
            size: Size {
                width: -1,
                height: -1,
            },
            major_dimension: 0,
            style: RadioBoxStyle::default(),
        }
    }

    pub fn with_id(mut self, id: Id) -> Self {
        self.id = id;
        self
    }

    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = label;
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

    pub fn with_major_dimension(mut self, dim: i32) -> Self {
        self.major_dimension = dim;
        self
    }

    pub fn with_style(mut self, style: RadioBoxStyle) -> Self {
        self.style = style;
        self
    }

    pub fn build(self) -> RadioBox {
        let parent_ptr = self.parent.map_or(ptr::null_mut(), |p| p.handle_ptr());
        let c_label = CString::new(self.label).expect("CString::new failed for label");

        let c_choices: Vec<CString> = self
            .choices
            .iter()
            .map(|&s| CString::new(s).expect("CString::new failed for choice"))
            .collect();
        let c_choices_ptrs: Vec<*const c_char> = c_choices.iter().map(|cs| cs.as_ptr()).collect();

        let ptr = unsafe {
            ffi::wxd_RadioBox_Create(
                parent_ptr,
                self.id,
                c_label.as_ptr(),
                self.pos.into(),
                self.size.into(),
                self.choices.len() as i32,
                c_choices_ptrs.as_ptr(),
                self.major_dimension,
                self.style.bits() as ffi::wxd_Style_t,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create wxRadioBox");
        }
        unsafe { RadioBox::from_ptr(ptr) }
    }
}

// --- Trait Implementations ---

impl WxWidget for RadioBox {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.window.as_ptr()
    }
}

impl WxEvtHandler for RadioBox {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.window.as_ptr() as *mut _
    }

    // The WxEvtHandler trait already provides a generic `bind` method:
    // fn bind<F>(&self, event_type: EventType, callback: F)
    // where
    //     F: FnMut(Event) + 'static, <--- This is what it should be using
    //
    // So, no specific `bind_radiobox` is needed here if widgets directly implement WxEvtHandler
    // or if they deref to a Window that implements it. Widgets usually call the trait method.
}

// No explicit Drop needed.

// --- RadioBoxStyle Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i64)]
pub enum RadioBoxStyle {
    /// Default layout (wxWidgets decides based on major dimension).
    Default = 0,
    /// Arrange items in columns primarily.
    SpecifyCols = ffi::WXD_RA_SPECIFY_COLS,
    /// Arrange items in rows primarily.
    SpecifyRows = ffi::WXD_RA_SPECIFY_ROWS,
}

impl RadioBoxStyle {
    pub fn bits(self) -> i64 {
        self as i64
    }
}

impl Default for RadioBoxStyle {
    fn default() -> Self {
        RadioBoxStyle::Default
    }
}

// RadioBox styles are typically not combined, but BitOr/Assign might be useful if other flags emerge.
impl BitOr for RadioBoxStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
    }
}

impl BitOrAssign for RadioBoxStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe { std::mem::transmute(self.bits() | rhs.bits()) };
    }
}

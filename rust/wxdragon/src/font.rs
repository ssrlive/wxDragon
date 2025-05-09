use std::ffi::CStr;
use wxdragon_sys as ffi;

/// Wrapper for wxFont.
#[derive(Clone)]
pub struct Font {
    ptr: *mut ffi::wxd_Font_t,
    owned: bool, // Track if this instance owns the pointer
}

unsafe impl Send for Font {}

impl Font {
    /// Creates a new default font.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_Font_Create() };
        Self { ptr, owned: true }
    }

    /// Create a Font wrapper from a raw pointer.
    /// # Safety
    /// The pointer must be a valid pointer to a wxFont.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_Font_t, owned: bool) -> Self {
        Self { ptr, owned }
    }

    /// Get the raw pointer.
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_Font_t {
        self.ptr
    }

    /// Get the point size of the font.
    pub fn get_point_size(&self) -> i32 {
        unsafe { ffi::wxd_Font_GetPointSize(self.ptr) }
    }

    /// Get the font family.
    pub fn get_family(&self) -> i32 {
        unsafe { ffi::wxd_Font_GetFamily(self.ptr) }
    }

    /// Get the font style.
    pub fn get_style(&self) -> i32 {
        unsafe { ffi::wxd_Font_GetStyle(self.ptr) }
    }

    /// Get the font weight.
    pub fn get_weight(&self) -> i32 {
        unsafe { ffi::wxd_Font_GetWeight(self.ptr) }
    }

    /// Get whether the font is underlined.
    pub fn is_underlined(&self) -> bool {
        unsafe { ffi::wxd_Font_GetUnderlined(self.ptr) }
    }

    /// Get the font face name.
    pub fn get_face_name(&self) -> String {
        let mut buffer = vec![0u8; 256]; // Reasonable initial buffer size
        let len = unsafe {
            ffi::wxd_Font_GetFaceName(
                self.ptr,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i32,
            )
        };

        if len > 0 {
            // Resize buffer to actual length + 1 for null terminator
            buffer.resize((len + 1) as usize, 0);
            let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const i8) };
            c_str.to_string_lossy().into_owned()
        } else {
            String::new()
        }
    }

    /// Check if the font is valid.
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_Font_IsOk(self.ptr) }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                ffi::wxd_Font_Destroy(self.ptr);
            }
        }
    }
}

// Font family constants
pub const FONTFAMILY_DEFAULT: i32 = 0;
pub const FONTFAMILY_DECORATIVE: i32 = 1;
pub const FONTFAMILY_ROMAN: i32 = 2;
pub const FONTFAMILY_SCRIPT: i32 = 3;
pub const FONTFAMILY_SWISS: i32 = 4;
pub const FONTFAMILY_MODERN: i32 = 5;
pub const FONTFAMILY_TELETYPE: i32 = 6;

// Font style constants
pub const FONTSTYLE_NORMAL: i32 = 0;
pub const FONTSTYLE_ITALIC: i32 = 1;
pub const FONTSTYLE_SLANT: i32 = 2;

// Font weight constants
pub const FONTWEIGHT_NORMAL: i32 = 0;
pub const FONTWEIGHT_LIGHT: i32 = 1;
pub const FONTWEIGHT_BOLD: i32 = 2;

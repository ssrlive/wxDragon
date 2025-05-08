// Rust wrappers for wxWidgets Sizers (moved to sizers/box_sizer.rs)

// use crate::window::WxWidget;
use crate::sizers::base::Sizer;
use crate::sizers::WxSizer as WxSizerTrait;
use std::marker::PhantomData;
use std::ops::Deref;
use wxdragon_sys as ffi;

// --- Sizer Orientation Constants ---
pub type Orientation = i32; // C API uses int (wxd_Orientation_t)
pub const VERTICAL: Orientation = ffi::WXD_VERTICAL as i32;
pub const HORIZONTAL: Orientation = ffi::WXD_HORIZONTAL as i32;
pub const BOTH: Orientation = ffi::WXD_BOTH as i32;

// --- Sizer Flag Constants ---
pub type SizerFlags = i32; // C API uses int (wxd_SizerFlags_t)
pub const LEFT: SizerFlags = ffi::WXD_LEFT as i32;
pub const RIGHT: SizerFlags = ffi::WXD_RIGHT as i32;
pub const TOP: SizerFlags = ffi::WXD_ALIGN_TOP as i32;
pub const BOTTOM: SizerFlags = ffi::WXD_ALIGN_BOTTOM as i32;
pub const ALL: SizerFlags = ffi::WXD_ALL as i32;
pub const EXPAND: SizerFlags = ffi::WXD_EXPAND as i32;
pub const ALIGN_LEFT: SizerFlags = ffi::WXD_ALIGN_LEFT as i32;
pub const ALIGN_RIGHT: SizerFlags = ffi::WXD_ALIGN_RIGHT as i32;
pub const ALIGN_CENTER_VERTICAL: SizerFlags = ffi::WXD_ALIGN_CENTRE_VERTICAL as i32;
pub const ALIGN_CENTER_HORIZONTAL: SizerFlags = ffi::WXD_ALIGN_CENTRE_HORIZONTAL as i32;
pub const ALIGN_CENTRE: SizerFlags = ffi::WXD_ALIGN_CENTRE as i32;
pub const SHAPED: SizerFlags = ffi::WXD_SHAPED as i32;
pub const FIXED_MINSIZE: SizerFlags = ffi::WXD_FIXED_MINSIZE as i32;
pub const BORDER_DEFAULT: SizerFlags = ffi::WXD_BORDER_DEFAULT as i32;
pub const BORDER_SIMPLE: SizerFlags = ffi::WXD_BORDER_SIMPLE as i32;

/// Represents a wxBoxSizer.
#[derive(Clone)]
pub struct BoxSizer {
    sizer_base: Sizer,
}

// Implement WxSizer for BoxSizer (delegates to Sizer)
impl WxSizerTrait for BoxSizer {
    fn as_sizer_ptr(&self) -> *mut ffi::wxd_Sizer_t {
        self.sizer_base.as_sizer_ptr()
    }
}

// Implement Deref to access base Sizer methods directly
impl Deref for BoxSizer {
    type Target = Sizer;
    fn deref(&self) -> &Self::Target {
        &self.sizer_base
    }
}

impl BoxSizer {
    pub fn builder(orientation: Orientation) -> BoxSizerBuilder {
        BoxSizerBuilder::new(orientation)
    }
}

/// Builder for [`BoxSizer`].
pub struct BoxSizerBuilder {
    orientation: Orientation,
    _marker: PhantomData<()>,
}

impl BoxSizerBuilder {
    fn new(orientation: Orientation) -> Self {
        Self {
            orientation,
            _marker: PhantomData,
        }
    }

    pub fn build(self) -> BoxSizer {
        let ptr = unsafe { ffi::wxd_BoxSizer_Create(self.orientation) };
        let sizer_base =
            unsafe { Sizer::from_ptr(ptr).expect("Failed to create base Sizer for BoxSizer") };
        BoxSizer { sizer_base }
    }
}

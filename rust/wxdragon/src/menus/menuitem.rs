// wxdragon/src/menus/menuitem.rs
//! wxMenuItem wrapper and related types

use wxdragon_sys as ffi;

// --- Standard Menu Item IDs ---
// Define explicitly as i32, casting from the ffi i64 type
pub const ID_EXIT: i32 = ffi::WXD_ID_EXIT as i32;
pub const ID_ABOUT: i32 = ffi::WXD_ID_ABOUT as i32;
pub const ITEM_NORMAL: i32 = ffi::WXD_ITEM_NORMAL as i32;
pub const ITEM_CHECK: i32 = ffi::WXD_ITEM_CHECK as i32;
pub const ITEM_RADIO: i32 = ffi::WXD_ITEM_RADIO as i32;
pub const ITEM_SEPARATOR: i32 = ffi::WXD_ITEM_SEPARATOR as i32;

// Often used ID for separators
pub const ID_SEPARATOR: i32 = ffi::WXD_ITEM_SEPARATOR as i32; // Use ITEM_SEPARATOR value

// --- Item Kind Enum ---
// Cast from ffi i64 constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ItemKind {
    Normal = ffi::WXD_ITEM_NORMAL as i32,
    Separator = ffi::WXD_ITEM_SEPARATOR as i32,
    Check = ffi::WXD_ITEM_CHECK as i32,
    Radio = ffi::WXD_ITEM_RADIO as i32,
}

impl From<ItemKind> for i32 {
    fn from(kind: ItemKind) -> Self {
        kind as i32
    }
}

/// Represents a wxMenuItem.
/// Note: This is usually a non-owning wrapper, as wxMenu manages item lifetime.
pub struct MenuItem {
    #[allow(dead_code)]
    ptr: *mut ffi::wxd_MenuItem_t, // Non-owning pointer
}

impl MenuItem {
    /// Creates a non-owning wrapper from a raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is valid and its lifetime is managed elsewhere (e.g., by wxMenu).
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_MenuItem_t) -> Self {
        MenuItem { ptr }
    }

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    #[allow(dead_code)]
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_MenuItem_t {
        self.ptr
    }

    // --- Methods to modify MenuItem state (if needed later) ---
    /*
    pub fn set_label(&self, label: &str) { ... }
    pub fn get_label(&self) -> String { ... }
    pub fn enable(&self, enable: bool) { ... }
    pub fn is_enabled(&self) -> bool { ... }
    pub fn check(&self, check: bool) { ... } // For Check/Radio items
    pub fn is_checked(&self) -> bool { ... } // For Check/Radio items
    */
}

// Note: No Drop impl here, as wxMenu takes ownership.

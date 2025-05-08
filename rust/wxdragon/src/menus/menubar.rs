//! wxMenuBar wrapper

use crate::menus::Menu;
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a wxMenuBar.
/// Note: Ownership is typically transferred to the Frame when SetMenuBar is called.
pub struct MenuBar {
    ptr: *mut ffi::wxd_MenuBar_t,
}

impl MenuBar {
    /// Creates a new menu bar using the builder pattern.
    pub fn builder() -> MenuBarBuilder {
        MenuBarBuilder::new()
    }

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    /// NOTE: This is needed internally by Frame::set_menu_bar.
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_MenuBar_t {
        self.ptr
    }
}

// Note: No Drop impl here, as wxFrame takes ownership via SetMenuBar.

// --- MenuBar Builder ---

/// Builder for [`MenuBar`].
#[derive(Default)]
pub struct MenuBarBuilder {
    style: i64,
    items: Vec<(Menu, String)>, // Store menu and title pairs
    _marker: PhantomData<()>,   // Optional: for generics later?
}

impl MenuBarBuilder {
    /// Creates a new, default builder.
    pub fn new() -> Self {
        Default::default() // style = 0, items = empty vec
    }

    /// Sets the style flags for the menu bar.
    pub fn with_style(mut self, style: i64) -> Self {
        self.style = style;
        self
    }

    /// Appends a menu to be added to the menu bar.
    /// Takes ownership of the `Menu` object.
    pub fn append(mut self, menu: Menu, title: &str) -> Self {
        self.items.push((menu, title.to_string()));
        self
    }

    /// Builds the `MenuBar`.
    ///
    /// # Panics
    /// Panics if the menu bar cannot be created.
    pub fn build(self) -> MenuBar {
        let ptr = unsafe { ffi::wxd_MenuBar_Create(self.style as ffi::wxd_Style_t) };
        if ptr.is_null() {
            panic!("Failed to create MenuBar");
        }
        let menubar = MenuBar { ptr };

        // Append all collected menus
        for (menu, title) in self.items {
            let title_c = CString::new(title).unwrap_or_default();
            let menu_ptr = unsafe { menu.as_ptr() };
            // Forget menu wrapper as wxMenuBar takes ownership
            std::mem::forget(menu);
            unsafe {
                ffi::wxd_MenuBar_Append(menubar.ptr, menu_ptr, title_c.as_ptr());
            }
        }

        menubar
    }
}

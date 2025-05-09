//! wxMenu wrapper

use crate::menus::menuitem::{ItemKind, MenuItem};
use crate::id::Id;
use std::ffi::CString;
use std::marker::PhantomData;
use wxdragon_sys as ffi;

/// Represents a wxMenu.
/// Note: Ownership is typically transferred to the MenuBar when Append is called.
pub struct Menu {
    ptr: *mut ffi::wxd_Menu_t,
}

impl Menu {
    /// Creates a new, empty menu using the builder pattern.
    pub fn builder() -> MenuBuilder {
        MenuBuilder::new()
    }

    /// Appends a menu item.
    /// Returns a wrapper for the created item (for potential modification), but ownership remains with the menu.
    pub fn append(
        &self,
        id: Id,
        item: &str,
        help_string: &str,
        kind: ItemKind,
    ) -> Option<MenuItem> {
        self.append_raw(id, item, help_string, kind)
    }

    /// Appends a separator.
    pub fn append_separator(&self) {
        self.append_separator_raw();
    }

    /// Returns the raw pointer.
    /// # Safety
    /// The caller must ensure the pointer is used correctly.
    pub(crate) unsafe fn as_ptr(&self) -> *mut ffi::wxd_Menu_t {
        self.ptr
    }

    // Make append private as it's called by builder
    fn append_raw(
        &self,
        id: Id,
        item: &str,
        help_string: &str,
        kind: ItemKind,
    ) -> Option<MenuItem> {
        let item_c = CString::new(item).unwrap_or_default();
        let help_c = CString::new(help_string).unwrap_or_default();
        let item_ptr = unsafe {
            ffi::wxd_Menu_Append(self.ptr, id, item_c.as_ptr(), help_c.as_ptr(), kind.into())
        };
        if item_ptr.is_null() {
            None
        } else {
            // Return a MenuItem wrapper, but don't give it ownership
            Some(unsafe { MenuItem::from_ptr(item_ptr) })
        }
    }

    // Make append_separator private as it's called by builder
    fn append_separator_raw(&self) {
        unsafe {
            ffi::wxd_Menu_AppendSeparator(self.ptr);
        }
    }
}

// Note: No Drop impl here, as wxMenuBar takes ownership via Append.

// --- Menu Builder ---

// Enum to represent actions to perform on the menu during build
enum MenuAction {
    AppendItem {
        id: Id,
        item: String,
        help: String,
        kind: ItemKind,
    },
    AppendSeparator,
    // TODO: Add AppendSubMenu if needed
}

/// Builder for [`Menu`].
#[derive(Default)]
pub struct MenuBuilder {
    actions: Vec<MenuAction>,
    _marker: PhantomData<()>,
}

impl MenuBuilder {
    /// Creates a new, default builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds an item to be appended to the menu.
    pub fn append_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Normal,
        });
        self
    }

    /// Adds a check item to be appended to the menu.
    pub fn append_check_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Check,
        });
        self
    }

    /// Adds a radio item to be appended to the menu.
    pub fn append_radio_item(mut self, id: Id, item: &str, help: &str) -> Self {
        self.actions.push(MenuAction::AppendItem {
            id,
            item: item.to_string(),
            help: help.to_string(),
            kind: ItemKind::Radio,
        });
        self
    }

    /// Adds a separator to be appended to the menu.
    pub fn append_separator(mut self) -> Self {
        self.actions.push(MenuAction::AppendSeparator);
        self
    }

    /// Builds the `Menu`.
    ///
    /// # Panics
    /// Panics if the menu cannot be created.
    pub fn build(self) -> Menu {
        // Pass default title (empty string) and default style (0)
        let title_c = CString::new("").unwrap();
        let style = 0i64;
        let ptr = unsafe { ffi::wxd_Menu_Create(title_c.as_ptr(), style as ffi::wxd_Style_t) };
        if ptr.is_null() {
            panic!("Failed to create Menu");
        }
        let menu = Menu { ptr };

        // Perform actions
        for action in self.actions {
            match action {
                MenuAction::AppendItem {
                    id,
                    item,
                    help,
                    kind,
                } => {
                    // We might ignore the returned MenuItem here, as the builder doesn't expose it.
                    let _ = menu.append_raw(id, &item, &help, kind);
                }
                MenuAction::AppendSeparator => {
                    menu.append_separator_raw();
                }
            }
        }
        menu
    }
}

/* This is a new file */
//! Safe wrapper for wxDirPickerCtrl.

use std::ffi::{c_longlong, CString};
use wxdragon_sys as ffi;

use crate::event::WxEvtHandler;
use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};

// --- Style enum using macro ---
widget_style_enum!(
    name: DirPickerCtrlStyle,
    doc: "Style flags for DirPickerCtrl widgets.",
    variants: {
        Default: ffi::WXD_DIRP_DEFAULT_STYLE, "Default style, often includes UseTextCtrl.",
        DirMustExist: ffi::WXD_DIRP_DIR_MUST_EXIST, "The directory must exist.",
        ChangeDir: ffi::WXD_DIRP_CHANGE_DIR, "Change the current working directory when a directory is selected.",
        UseTextCtrl: ffi::WXD_DIRP_USE_TEXTCTRL, "Use a text control to display the selected directory."
    },
    default_variant: Default
);

// --- DirPickerCtrl ---
#[derive(Clone)]
pub struct DirPickerCtrl {
    window: Window, // Embed Window
}

impl DirPickerCtrl {
    /// Creates a new DirPickerCtrlBuilder.
    pub fn builder(parent: &dyn WxWidget) -> DirPickerCtrlBuilder {
        DirPickerCtrlBuilder::new(parent)
    }

    /// Gets the currently selected path.
    pub fn get_path(&self) -> String {
        unsafe {
            let c_str = ffi::wxd_DirPickerCtrl_GetPath(
                self.window.as_ptr() as *mut ffi::wxd_DirPickerCtrl_t
            );
            if c_str.is_null() {
                String::new()
            } else {
                // Free the string using wxd_free_string when we're done with it
                let rust_str = CString::from_raw(c_str as *mut _)
                    .to_string_lossy()
                    .into_owned();
                rust_str
            }
        }
    }

    /// Sets the currently selected path.
    pub fn set_path(&self, path: &str) {
        let c_path = CString::new(path).expect("CString::new failed for path");
        unsafe {
            ffi::wxd_DirPickerCtrl_SetPath(
                self.window.as_ptr() as *mut ffi::wxd_DirPickerCtrl_t,
                c_path.as_ptr(),
            );
        }
    }

    /// Creates a DirPickerCtrl from a raw pointer.
    /// # Safety
    /// The pointer must be a valid `wxd_DirPickerCtrl_t`.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_DirPickerCtrl_t) -> Self {
        DirPickerCtrl {
            window: Window::from_ptr(ptr as *mut ffi::wxd_Window_t),
        }
    }
}

// Use the widget_builder macro to generate the DirPickerCtrlBuilder implementation
widget_builder!(
    name: DirPickerCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: DirPickerCtrlStyle,
    fields: {
        message: String = "Select a directory".to_string(),
        path: String = String::new()
    },
    build_impl: |slf| {
        assert!(!slf.parent.handle_ptr().is_null(), "DirPickerCtrl requires a parent");

        let c_message = CString::new(&slf.message[..]).expect("CString::new failed for message");
        let c_path = CString::new(&slf.path[..]).expect("CString::new failed for path");

        let ptr = unsafe {
            ffi::wxd_DirPickerCtrl_Create(
                slf.parent.handle_ptr(),
                slf.id,
                c_message.as_ptr(),
                c_path.as_ptr(),
                slf.pos.into(),
                slf.size.into(),
                slf.style.bits() as c_longlong,
            )
        };
        if ptr.is_null() {
            panic!("Failed to create DirPickerCtrl: FFI returned null pointer.");
        } else {
            unsafe { DirPickerCtrl::from_ptr(ptr) }
        }
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(DirPickerCtrl, window, Window);

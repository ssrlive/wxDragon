use crate::implement_widget_traits_with_target;
use crate::prelude::*;
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget};
use std::ffi::CString;
use wxdragon_sys as ffi;

// Default wildcard pattern for FileCtrl
const ALL_FILES_PATTERN: &str = "*.*";

// Define the FileCtrlStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: FileCtrlStyle,
    doc: "Style flags for `FileCtrl`.",
    variants: {
        Open: ffi::WXD_FC_OPEN, "Default style for opening files.",
        Save: ffi::WXD_FC_SAVE, "For saving files.",
        Multiple: ffi::WXD_FC_MULTIPLE, "Allow multiple files to be selected.",
        NoShowHidden: ffi::WXD_FC_NOSHOWHIDDEN, "Don't show hidden files."
    },
    default_variant: Open
);

#[derive(Clone)]
pub struct FileCtrl {
    window: Window, // Composition: FileCtrl IS a Window
}

impl FileCtrl {
    pub fn builder(parent: &dyn WxWidget) -> FileCtrlBuilder {
        FileCtrlBuilder::new(parent)
    }

    // Create a new FileCtrl from a window and parent pointer
    // This is intended for internal use
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: i64,
        pos: Point,
        size: Size,
        name: &str,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "FileCtrl requires a parent");
        let c_default_dir =
            CString::new(default_directory).expect("CString::new failed for default_directory");
        let c_default_filename =
            CString::new(default_filename).expect("CString::new failed for default_filename");
        let c_wild_card = CString::new(wild_card).expect("CString::new failed for wild_card");
        let c_name = CString::new(name).expect("CString::new failed for name");

        let raw_ptr = unsafe {
            ffi::wxd_FileCtrl_Create(
                parent_ptr,
                id,
                c_default_dir.as_ptr(),
                c_default_filename.as_ptr(),
                c_wild_card.as_ptr(),
                style,
                pos.x,
                pos.y,
                size.width,
                size.height,
                c_name.as_ptr(),
            )
        };
        if raw_ptr.is_null() {
            panic!("Failed to create wxFileCtrl via FFI");
        }
        // Cast the concrete FileCtrl pointer to the base Window pointer for the wrapper
        let window = unsafe { Window::from_ptr(raw_ptr as *mut ffi::wxd_Window_t) };
        FileCtrl { window }
    }
}

// Use the widget_builder macro to generate the FileCtrlBuilder implementation
widget_builder!(
    name: FileCtrl,
    parent_type: &'a dyn WxWidget,
    style_type: FileCtrlStyle,
    fields: {
        default_directory: String = String::new(),
        default_filename: String = String::new(),
        wild_card: String = ALL_FILES_PATTERN.to_string(),
        name: String = "FileCtrl".to_string()
    },
    build_impl: |slf| {
        FileCtrl::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            &slf.default_directory,
            &slf.default_filename,
            &slf.wild_card,
            slf.style.bits(),
            slf.pos,
            slf.size,
            &slf.name,
        )
    }
);

// Use the implement_widget_traits_with_target macro to implement traits
implement_widget_traits_with_target!(FileCtrl, window, Window);

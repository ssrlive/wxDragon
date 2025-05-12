use crate::event::WxEvtHandler;
use crate::implement_widget_traits;
use crate::prelude::*; // Use prelude
use crate::widget_builder;
use crate::widget_style_enum;
use crate::window::{Window, WxWidget}; // Make sure WxEvtHandler is imported
                                       // Remove specific imports covered by prelude
                                       // use crate::{Id, Point, Size};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wxdragon_sys as ffi; // ADDED for enum bitwise operations

/// Represents a wxButton.
#[derive(Clone)]
pub struct Button {
    window: Window, // Composition: Button IS a Window
    // Store parent pointer to manage drop behavior
    // Allow dead_code because it's used implicitly by the Drop logic.
    #[allow(dead_code)]
    parent_ptr: *mut ffi::wxd_Window_t,
}

impl Button {
    /// Creates a new `ButtonBuilder` for constructing a button.
    pub fn builder(parent: &dyn WxWidget) -> ButtonBuilder {
        ButtonBuilder::new(parent)
    }

    /// Creates a new Button from a raw window and parent pointer.
    /// This is intended for internal use by other widget wrappers that compose Button.
    pub(crate) fn new_from_composition(window: Window, parent_ptr: *mut ffi::wxd_Window_t) -> Self {
        Self { window, parent_ptr }
    }

    /// Creates a new Button (low-level constructor used by the builder)
    fn new_impl(
        parent_ptr: *mut ffi::wxd_Window_t,
        id: Id,
        label: &str,
        pos: Point,
        size: Size,
        style: i64,
    ) -> Self {
        assert!(!parent_ptr.is_null(), "Button requires a parent");
        let c_label = CString::new(label).expect("CString::new failed");

        let ptr = unsafe {
            ffi::wxd_Button_Create(
                parent_ptr,
                id,
                c_label.as_ptr(),
                pos.into(),
                size.into(),
                style as ffi::wxd_Style_t,
            )
        };

        if ptr.is_null() {
            panic!("Failed to create Button widget");
        } else {
            // Cast the concrete Button pointer to the base Window pointer for the wrapper
            let window = unsafe { Window::from_ptr(ptr as *mut ffi::wxd_Window_t) };
            Button {
                window,
                parent_ptr,
            }
        }
    }

    /// Gets the button's label.
    pub fn get_label(&self) -> String {
        let mut buffer: [c_char; 256] = [0; 256]; // Reasonable buffer size
        let len_needed = unsafe {
            ffi::wxd_Button_GetLabel(
                self.window.as_ptr() as *mut ffi::wxd_Button_t,
                buffer.as_mut_ptr(),
                buffer.len() as i32,
            )
        };

        if len_needed > 0 && (len_needed as usize) <= buffer.len() {
            unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            }
        } else if len_needed > (buffer.len() as i32) {
            // Buffer too small, try again with required size
            let mut vec_buffer: Vec<c_char> = vec![0; len_needed as usize];
            let len_needed_2 = unsafe {
                ffi::wxd_Button_GetLabel(
                    self.window.as_ptr() as *mut ffi::wxd_Button_t,
                    vec_buffer.as_mut_ptr(),
                    vec_buffer.len() as i32,
                )
            };
            if len_needed_2 == len_needed {
                unsafe {
                    CStr::from_ptr(vec_buffer.as_ptr())
                        .to_string_lossy()
                        .into_owned()
                }
            } else {
                // Something went wrong
                String::new()
            }
        } else {
            // Error or empty label
            String::new()
        }
    }
}

// Use the widget_builder macro to generate the ButtonBuilder implementation
widget_builder!(
    name: Button,
    parent_type: &'a dyn WxWidget,
    style_type: ButtonStyle,
    fields: {},
    build_impl: |slf| {
        let parent_ptr = slf.parent.handle_ptr();
        Button::new_impl(
            parent_ptr,
            slf.id,
            &slf.label,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

// Apply common trait implementations for Button
implement_widget_traits!(Button, window);

// Define the ButtonStyle enum using the widget_style_enum macro
widget_style_enum!(
    name: ButtonStyle,
    doc: "Style flags for `Button`.",
    variants: {
        Default: 0, "Default style (no specific alignment, standard border).",
        Left: ffi::WXD_BU_LEFT, "Align label to the left.",
        Top: ffi::WXD_BU_TOP, "Align label to the top.",
        Right: ffi::WXD_BU_RIGHT, "Align label to the right.",
        Bottom: ffi::WXD_BU_BOTTOM, "Align label to the bottom.",
        ExactFit: ffi::WXD_BU_EXACTFIT, "Button size will be adjusted to exactly fit the label.",
        NoText: ffi::WXD_BU_NOTEXT, "Do not display the label string (useful for buttons with only an image).",
        BorderNone: ffi::WXD_BORDER_NONE, "No border.",
        BorderSimple: ffi::WXD_BORDER_SIMPLE, "A simple border (rarely used for buttons, which have a default look)."
    },
    default_variant: Default
);

// use crate::event::WxEvtHandler; // This might be unused if all macros import it directly.

/// Macros to simplify implementation of common patterns in the codebase.

/// Creates a builder pattern for widgets
///
/// This macro generates a builder struct with standard fields and methods
/// following the WxDragon convention.
///
/// # Parameters
///
/// * `name` - The name of the widget (e.g., `Button`, `CheckBox`)
/// * `parent_type` - The type of the parent parameter (e.g., `&'a dyn WxWidget`)
/// * `style_type` - The type of the style parameter (e.g., `ButtonStyle`)
/// * `fields` - Additional fields for the builder, including label if needed
/// * `build_impl` - A closure that implements the build logic
///
/// # Example
///
/// ```ignore
/// widget_builder!(
///     name: Button,
///     parent_type: &'a dyn WxWidget,
///     style_type: ButtonStyle,
///     fields: {
///         label: String = String::new()
///     },
///     build_impl: |slf| {
///         let parent_ptr = slf.parent.handle_ptr();
///         Button::new_impl(
///             parent_ptr,
///             slf.id,
///             &slf.label,
///             slf.pos,
///             slf.size,
///             slf.style.bits(),
///         )
///     }
/// );
/// ```
#[macro_export]
macro_rules! widget_builder {
    (
        name: $name:ident,
        parent_type: &'a dyn WxWidget,
        style_type: $style_type:ty,
        fields: {
            $(
                $field_name:ident: $field_type:ty $(= $field_default:expr)?
            ),*
        },
        build_impl: |$self_param:ident| $build_impl:block
    ) => {
        paste::paste! {
            #[derive(Clone)]
            pub struct [<$name Builder>]<'a> {
                parent: &'a dyn WxWidget,
                id: Id,
                pos: Point,
                size: Size,
                style: $style_type,
                $(
                    $field_name: $field_type,
                )*
            }

            impl<'a> [<$name Builder>]<'a> {
                pub fn new(parent: &'a dyn WxWidget) -> Self {
                    Self {
                        parent,
                        id: crate::id::ID_ANY as Id,
                        pos: crate::geometry::DEFAULT_POSITION,
                        size: crate::geometry::DEFAULT_SIZE,
                        style: <$style_type>::Default,
                        $(
                            $field_name: $crate::__widget_builder_default!($($field_default)?),
                        )*
                    }
                }

                /// Sets the window identifier.
                pub fn with_id(mut self, id: Id) -> Self {
                    self.id = id;
                    self
                }

                /// Sets the position.
                pub fn with_pos(mut self, pos: Point) -> Self {
                    self.pos = pos;
                    self
                }

                /// Sets the size.
                pub fn with_size(mut self, size: Size) -> Self {
                    self.size = size;
                    self
                }

                /// Sets the window style flags.
                pub fn with_style(mut self, style: $style_type) -> Self {
                    self.style = style;
                    self
                }

                $(
                    $crate::__widget_builder_field_method!($field_name: $field_type);
                )*

                /// Builds the widget.
                pub fn build(self) -> $name {
                    let build_fn = |$self_param: [<$name Builder>]<'a>| $build_impl;
                    build_fn(self)
                }
            }
        }
    };
}

/// Implements common widget traits.
///
/// This macro generates implementations for:
/// - WxWidget trait
/// - Deref/DerefMut to Window
/// - WxEvtHandler trait
/// - Drop implementation with empty body (for child widgets)
///
/// # Parameters
///
/// * `name` - The name of the widget struct
/// * `field` - The name of the field within the struct that is a Window
///
/// # Example
///
/// ```ignore
/// implement_widget_traits!(MyWidget, window_field);
/// ```
#[macro_export]
macro_rules! implement_widget_traits {
    ($widget_name:ident, $window_field:ident) => {
        // Don't import WxEvtHandler here as modules might already have it imported
        // use $crate::event::WxEvtHandler;

        impl $crate::window::WxWidget for $widget_name {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.$window_field.handle_ptr()
            }
        }

        impl std::ops::Deref for $widget_name {
            type Target = $crate::window::Window;
            fn deref(&self) -> &Self::Target {
                &self.$window_field
            }
        }

        impl std::ops::DerefMut for $widget_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$window_field
            }
        }

        impl $crate::event::WxEvtHandler for $widget_name {
            unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
                self.$window_field.get_event_handler_ptr()
            }
        }

        impl Drop for $widget_name {
            fn drop(&mut self) {
                // Child windows are managed by their parent in wxWidgets,
                // so no explicit cleanup is needed here.
            }
        }
    };
}

/// Implements common widget traits with a custom target type for Deref
///
/// This macro generates implementations for:
/// - WxWidget trait
/// - Deref/DerefMut to the specified target type
/// - WxEvtHandler trait
/// - Drop implementation with empty body (for child widgets)
///
/// # Parameters
///
/// * `name` - The name of the widget struct
/// * `field` - The name of the field within the struct that implements WxWidget
/// * `target_type` - The target type for Deref/DerefMut implementations
///
/// # Example
///
/// ```ignore
/// // For a widget that wraps a Window:
/// implement_widget_traits_with_target!(MyWidget, window, Window);
///
/// // For a widget that wraps another widget type:
/// implement_widget_traits_with_target!(MyCompositeWidget, inner_widget, OtherWidgetType);
/// ```
#[macro_export]
macro_rules! implement_widget_traits_with_target {
    ($widget_name:ident, $window_field:ident, $target_ty:ty) => {
        // Don't import WxEvtHandler here as modules might already have it imported
        // use $crate::event::WxEvtHandler;

        impl $crate::window::WxWidget for $widget_name {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.$window_field.handle_ptr()
            }
        }

        impl std::ops::Deref for $widget_name {
            type Target = $target_ty;
            fn deref(&self) -> &Self::Target {
                &self.$window_field
            }
        }

        impl std::ops::DerefMut for $widget_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$window_field
            }
        }

        impl $crate::event::WxEvtHandler for $widget_name {
            unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
                self.$window_field.get_event_handler_ptr()
            }
        }

        impl Drop for $widget_name {
            fn drop(&mut self) {
                // Child windows are managed by their parent in wxWidgets,
                // so no explicit cleanup is needed here.
            }
        }
    };
}

/// Defines a style enum for a widget with standard implementations
///
/// This macro creates a style enum with variants from wxWidgets constants
/// and implements common traits like Default, BitOr, BitOrAssign, and bits().
///
/// # Parameters
///
/// * `name` - The name of the style enum (e.g., `ButtonStyle`, `RadioBoxStyle`)
/// * `doc` - Documentation for the enum (e.g., "Style flags for Button")
/// * `variants` - A list of variants with their names, wxWidgets constants, and docs
/// * `default_variant` - The name of the variant to use as Default
///
/// # Example
///
/// ```ignore
/// widget_style_enum!(
///     name: ButtonStyle,
///     doc: "Style flags for Button.",
///     variants: {
///         Default: 0, "Default style (no specific alignment).",
///         Left: ffi::WXD_BU_LEFT, "Align label to the left.",
///         Right: ffi::WXD_BU_RIGHT, "Align label to the right."
///     },
///     default_variant: Default
/// );
/// ```
#[macro_export]
macro_rules! widget_style_enum {
    (
        name: $name:ident,
        doc: $doc:expr,
        variants: {
            $(
                $variant:ident: $value:expr, $variant_doc:expr
            ),+
        },
        default_variant: $default:ident
    ) => {
        #[doc = $doc]
        #[doc = "\n\nThese flags can be combined using the bitwise OR operator (`|`)."]
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(i64)]
        pub enum $name {
            $(
                #[doc = $variant_doc]
                $variant = $value,
            )+
        }

        impl $name {
            /// Returns the raw integer value of the style.
            pub fn bits(self) -> i64 {
                self as i64
            }

            // Add constant for compatibility with existing code
            #[allow(non_upper_case_globals)]
            pub const Default: $name = $name::$default;
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default
            }
        }

        impl std::ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                unsafe { std::mem::transmute(self.bits() | rhs.bits()) }
            }
        }

        impl std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                unsafe {
                    *self = std::mem::transmute(self.bits() | rhs.bits());
                }
            }
        }
    };
}

// Helper macro for generating appropriate setter methods based on field type
#[doc(hidden)]
#[macro_export]
macro_rules! __widget_builder_field_method {
    // Special case for String fields - allow &str parameters and convert to String
    // This applies to any field with type String, regardless of name
    ($field_name:ident: String) => {
        paste::paste! {
            /// Sets the $field_name.
            #[allow(non_snake_case)]
            pub fn [<with_ $field_name>](mut self, $field_name: &str) -> Self {
                self.$field_name = $field_name.to_string();
                self
            }
        }
    };
    // Default case for all other fields - generate a method that takes the same type as the field
    ($field_name:ident: $field_type:ty) => {
        paste::paste! {
            /// Sets the $field_name.
            #[allow(non_snake_case)]
            pub fn [<with_ $field_name>](mut self, $field_name: $field_type) -> Self {
                self.$field_name = $field_name;
                self
            }
        }
    };
}

// Helper macro for default values in fields
#[doc(hidden)]
#[macro_export]
macro_rules! __widget_builder_default {
    () => {
        Default::default()
    };
    ($expr:expr) => {
        $expr
    };
}

/// Implements XrcSupport trait for a widget with custom field initialization
///
/// This macro generates the XrcSupport implementation for widgets, handling
/// different field patterns that widgets may have.
///
/// # Parameters
///
/// * `widget_name` - The name of the widget struct
/// * `fields` - Block containing field initialization expressions
///
/// # Example
///
/// ```ignore
/// // For widgets with just a window field:
/// impl_xrc_support!(TextCtrl, {
///     window
/// });
///
/// // For widgets with window and parent_ptr fields:
/// impl_xrc_support!(Button, {
///     window,
///     parent_ptr: std::ptr::null_mut()
/// });
/// ```
#[macro_export]
macro_rules! impl_xrc_support {
    // Handle the simple case where only window field is specified
    ($widget_name:ident, { window }) => {
        impl $crate::xrc::XrcSupport for $widget_name {
            fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let window = unsafe { $crate::window::Window::from_ptr(ptr) };
                Self { window }
            }
        }
    };
    // Handle the more complex case with additional fields
    ($widget_name:ident, { window, $($field_name:ident: $field_value:expr),* $(,)? }) => {
        impl $crate::xrc::XrcSupport for $widget_name {
            fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let window = unsafe { $crate::window::Window::from_ptr(ptr) };
                Self {
                    window,
                    $(
                        $field_name: $field_value,
                    )*
                }
            }
        }
    };
}

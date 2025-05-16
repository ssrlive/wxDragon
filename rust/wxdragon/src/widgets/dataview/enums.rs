//! Enums for DataView components.

use crate::widget_style_enum;
use wxdragon_sys as ffi;

// Define cell mode enum
widget_style_enum!(
    name: DataViewCellMode,
    doc: "Mode flags for DataView cell rendering and interaction.",
    variants: {
        Inert: ffi::WXD_DATAVIEW_CELL_INERT, "Cell cannot be edited or activated.",
        Activatable: ffi::WXD_DATAVIEW_CELL_ACTIVATABLE, "Cell can be activated (clicked) but not edited.",
        Editable: ffi::WXD_DATAVIEW_CELL_EDITABLE, "Cell can be edited."
    },
    default_variant: Inert
);

// Define alignment enum using the macro
widget_style_enum!(
    name: DataViewAlign,
    doc: "Alignment flags for DataView cells. These flags can be combined to specify both horizontal and vertical alignment.",
    variants: {
        Left: ffi::WXD_ALIGN_LEFT, "Align cell content to the left.",
        Right: ffi::WXD_ALIGN_RIGHT, "Align cell content to the right.",
        Center: ffi::WXD_ALIGN_CENTER, "Center cell content horizontally.",
        CenterHorizontal: ffi::WXD_ALIGN_CENTRE_HORIZONTAL, "Center cell content horizontally.",
        Bottom: ffi::WXD_ALIGN_BOTTOM, "Align cell content to the bottom.",
        CenterVertical: ffi::WXD_ALIGN_CENTER_VERTICAL, "Center cell content vertically."
    },
    default_variant: Left
);

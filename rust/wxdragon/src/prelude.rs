// --- Core Types & Traits ---
pub use crate::app::{main, WxdAppHandle};
pub use crate::color::{Colour, colours};
pub use crate::datetime::DateTime;
pub use crate::geometry::{Point, Rect, Size, DEFAULT_POSITION, DEFAULT_SIZE};
pub use crate::id::{Id, ID_ANY};
pub use crate::types::Style;
pub use crate::event::{Event, EventType, WxEvtHandler};
pub use crate::sizers::WxSizer;
pub use crate::window::{WindowUserData, WxWidget};

// --- Sizers ---
pub use crate::sizers::box_sizer::{BoxSizer, BoxSizerBuilder, Orientation, SizerFlags};
// Sizer Flags/Constants
pub use crate::sizers::box_sizer::{
    ALIGN_CENTER_HORIZONTAL, ALIGN_CENTER_VERTICAL, ALIGN_LEFT, ALIGN_RIGHT, ALL, BORDER_DEFAULT,
    BORDER_SIMPLE, BOTTOM, EXPAND, FIXED_MINSIZE, HORIZONTAL, LEFT, RIGHT, SHAPED, TOP, VERTICAL,
};
// ADDED: StaticBoxSizer
pub use crate::sizers::staticbox_sizer::{StaticBoxSizer, StaticBoxSizerBuilder};
// ADDED: FlexGridSizer
pub use crate::sizers::flex_grid_sizer::{FlexGridSizer, FlexGridSizerBuilder};

// --- Widgets & Builders ---
pub use crate::widgets::activity_indicator::{ActivityIndicator, ActivityIndicatorBuilder};
pub use crate::widgets::animation_ctrl::{AnimationCtrl, AnimationCtrlBuilder};
pub use crate::widgets::bitmapbutton::{BitmapButton, BitmapButtonBuilder};
pub use crate::widgets::button::{Button, ButtonBuilder, ButtonStyle};
pub use crate::widgets::checkbox::{CheckBox, CheckBoxBuilder, CheckBoxStyle};
pub use crate::widgets::checklistbox::{CheckListBox, CheckListBoxBuilder};
pub use crate::widgets::choice::{Choice, ChoiceBuilder};
pub use crate::widgets::combobox::{ComboBox, ComboBoxBuilder, ComboBoxStyle};
pub use crate::widgets::frame::{Frame, FrameBuilder, FrameStyle};
pub use crate::widgets::gauge::{Gauge, GaugeBuilder, GaugeStyle};
pub use crate::widgets::listbox::{ListBox, ListBoxBuilder, ListBoxStyle};
pub use crate::widgets::panel::{Panel, PanelBuilder, PanelStyle};
pub use crate::widgets::radio_button::{RadioButton, RadioButtonBuilder, RadioButtonStyle};
pub use crate::widgets::static_text::{StaticText, StaticTextBuilder, StaticTextStyle};
pub use crate::widgets::staticbox::{StaticBox, StaticBoxBuilder};
pub use crate::widgets::textctrl::{TextCtrl, TextCtrlBuilder, TextCtrlStyle};
pub use crate::widgets::togglebutton::{ToggleButton, ToggleButtonBuilder};
pub use crate::widgets::treectrl::{TreeCtrl, TreeCtrlBuilder, TreeCtrlStyle};
// ADDED: Slider
pub use crate::widgets::slider::{Slider, SliderBuilder, SliderStyle};
// ADDED: SpinCtrl
pub use crate::widgets::spinctrl::{SpinCtrl, SpinCtrlBuilder, SpinCtrlStyle};
// ADDED: SpinButton
pub use crate::widgets::spinbutton::{SpinButton, SpinButtonBuilder, SpinButtonStyle};
// ADDED: Notebook
pub use crate::widgets::notebook::{Notebook, NotebookBuilder, NotebookStyle};
// ADDED: SplitterWindow
pub use crate::widgets::splitterwindow::{SplitterWindow, SplitterWindowBuilder, SplitterWindowStyle};
// ADDED: ScrolledWindow
pub use crate::widgets::scrolled_window::{ScrolledWindow, ScrolledWindowBuilder};
// ADDED: StatusBar
pub use crate::widgets::statusbar::{StatusBar, StatusBarBuilder};
// ADDED: ToolBar
pub use crate::widgets::toolbar::ToolBar;
// ADDED: ListCtrl
pub use crate::widgets::list_ctrl::{
    ListCtrl, ListCtrlBuilder, ListCtrlStyle, 
    ListColumnFormat, ListItemState, ListNextItemFlag
};
// ADDED: RadioBox
pub use crate::widgets::radiobox::RadioBox;
// ADDED: BitmapComboBox
pub use crate::widgets::bitmapcombobox::{BitmapComboBox, BitmapComboBoxBuilder};
// ADDED: CommandLinkButton
pub use crate::widgets::command_link_button::{CommandLinkButton, CommandLinkButtonBuilder};
pub use crate::widgets::file_ctrl::{FileCtrl, FileCtrlBuilder, FileCtrlStyle};

// --- ADDED: Menus ---
pub use crate::menus::{ItemKind, Menu, MenuBar, MenuItem};
// REMOVED: No longer exporting ITEM_* constants directly
// pub use crate::menus::menuitem::{ITEM_NORMAL, ITEM_SEPARATOR, ITEM_CHECK, ITEM_RADIO};
// ADDED: Export standard IDs
pub use crate::menus::menuitem::{ID_ABOUT, ID_EXIT, ID_SEPARATOR};
// pub use crate::id::ID_ANY; // REMOVED: Duplicate export
// pub use crate::id::ID_HIGHEST; // REMOVED: Already exported at the end of the file
// ADDED: Bitmap
pub use crate::bitmap::Bitmap;

// Replace old ArtProvider const exports with new Enum exports
// pub use crate::art_provider::{
//     ArtProvider, ART_ADD_BOOKMARK, ART_BUTTON, ... (all old consts listed here) ...
// };
pub use crate::art_provider::{ArtClient, ArtId, ArtProvider};

// Re-export StaticLine - remove old consts
pub use crate::widgets::static_line::{StaticLine, StaticLineBuilder};
// Re-export ScrollBar - remove old consts
pub use crate::widgets::scrollbar::{ScrollBar, ScrollBarBuilder};

// ADDED: Re-export ColourPickerCtrl and related items
pub use crate::widgets::colourpickerctrl::{ColourPickerCtrl, ColourPickerCtrlBuilder};

// ADDED: Re-export DatePickerCtrl, DatePickerCtrlBuilder
pub use crate::widgets::datepickerctrl::{DatePickerCtrl, DatePickerCtrlBuilder};

// ADDED: Re-export Treebook
pub use crate::widgets::treebook::{Treebook, TreebookBuilder};

// ADDED: Re-export SearchCtrl
pub use crate::widgets::search_ctrl::{SearchCtrl, SearchCtrlBuilder};

// ADDED: Re-export HyperlinkCtrl
pub use crate::widgets::hyperlink_ctrl::{HyperlinkCtrl, HyperlinkCtrlBuilder};

// ADDED: Re-export SpinCtrlDouble
pub use crate::widgets::spinctrldouble::{SpinCtrlDouble, SpinCtrlDoubleBuilder};

// ADDED: Re-export CalendarCtrl
pub use crate::widgets::calendar_ctrl::{CalendarCtrl, CalendarCtrlBuilder};

// ADDED: Re-export StaticBitmap
pub use crate::widgets::static_bitmap::{StaticBitmap, StaticBitmapBuilder};

// ADDED: Dialog and MessageDialog
pub use crate::dialogs::colour_dialog::ColourDialog;
pub use crate::dialogs::font_dialog::FontDialog;
pub use crate::dialogs::message_dialog::{
    MessageDialog, MessageDialogBuilder, CANCEL, CENTRE, ICON_ERROR, ICON_EXCLAMATION, ICON_HAND,
    ICON_INFORMATION, ICON_QUESTION, ICON_WARNING, NO, OK, YES,
};
pub use crate::dialogs::text_entry_dialog::{TextEntryDialog, TextEntryDialogBuilder};
pub use crate::dialogs::Dialog;

// ADDED: Font enums (FontFamily, FontStyle, FontWeight) and FontData
// REMOVE old Font constants, export new enums
pub use crate::font::{Font, FontFamily, FontStyle, FontWeight};
pub use crate::font_data::FontData;
// The old const exports like FONTFAMILY_DEFAULT, FONTSTYLE_NORMAL, FONTWEIGHT_BOLD are removed.
// Example of what is being removed by not re-adding:
// pub use crate::font::{
//     FONTFAMILY_DECORATIVE, FONTFAMILY_DEFAULT, FONTFAMILY_MODERN, FONTFAMILY_ROMAN, 
//     FONTFAMILY_SCRIPT, FONTFAMILY_SWISS, FONTFAMILY_TELETYPE, FONTSTYLE_ITALIC, 
//     FONTSTYLE_NORMAL, FONTSTYLE_SLANT, FONTWEIGHT_BOLD, FONTWEIGHT_LIGHT, FONTWEIGHT_NORMAL,
// };

// ADDED: ProgressDialog
pub use crate::dialogs::progress_dialog::ProgressDialog;

// ADDED: File/Dir Picker Controls, their builders, and style enums
pub use crate::widgets::dir_picker_ctrl::{DirPickerCtrl, DirPickerCtrlBuilder, DirPickerCtrlStyle};
pub use crate::widgets::file_picker_ctrl::{FilePickerCtrl, FilePickerCtrlBuilder, FilePickerCtrlStyle};

// REPLACE OLD FontPickerCtrl exports with new enum export
pub use crate::widgets::{FontPickerCtrl, FontPickerCtrlBuilder, FontPickerCtrlStyle};
// The old block was:
// pub use crate::widgets::{
//     FontPickerCtrl, FontPickerCtrlBuilder, FNTP_DEFAULT_STYLE, FNTP_FONTDESC_AS_LABEL,
//     FNTP_USEFONT_FOR_LABEL, FNTP_USE_TEXTCTRL,
// };

// NotificationMessage
pub use crate::widgets::notification_message::{
    NotificationMessage,
    NotificationMessageBuilder,
    // ICON_INFORMATION, ICON_WARNING, ICON_ERROR, ICON_QUESTION, // Removed to avoid conflict with message_dialog exports
    TIMEOUT_AUTO,
    TIMEOUT_NEVER,
};

// Re-export other common constants or types if needed
// pub use crate::base::ID_ANY; // Example if needed directly
pub use crate::id::ID_HIGHEST; // ADDED ID_HIGHEST

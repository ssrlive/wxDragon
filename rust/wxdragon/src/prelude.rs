// --- Core Types & Traits ---
pub use crate::app::{main, WxdAppHandle};
pub use crate::base::{Point, Size};
pub use crate::event::{Event, EventType, WxEvtHandler};
pub use crate::id::Id;
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
pub use crate::widgets::gauge::{Gauge, GaugeBuilder};
pub use crate::widgets::listbox::{ListBox, ListBoxBuilder, ListBoxStyle};
pub use crate::widgets::panel::{Panel, PanelBuilder, PanelStyle};
pub use crate::widgets::radio_button::{RadioButton, RadioButtonBuilder, RadioButtonStyle};
pub use crate::widgets::static_text::{StaticText, StaticTextBuilder, StaticTextStyle};
pub use crate::widgets::staticbox::{StaticBox, StaticBoxBuilder};
pub use crate::widgets::textctrl::{TextCtrl, TextCtrlBuilder, TextCtrlStyle};
pub use crate::widgets::togglebutton::{ToggleButton, ToggleButtonBuilder};
pub use crate::widgets::treectrl::{TreeCtrl, TreeCtrlBuilder};
// ADDED: Slider
pub use crate::widgets::slider::{Slider, SliderBuilder};
// ADDED: SpinCtrl
pub use crate::widgets::spinctrl::{SpinCtrl, SpinCtrlBuilder};
// ADDED: SpinButton
pub use crate::widgets::spinbutton::{SpinButton, SpinButtonBuilder};
// ADDED: Notebook
pub use crate::widgets::notebook::{Notebook, NotebookBuilder};
// ADDED: SplitterWindow
pub use crate::widgets::splitterwindow::{SplitterWindow, SplitterWindowBuilder};
// ADDED: ScrolledWindow
pub use crate::widgets::scrolled_window::{ScrolledWindow, ScrolledWindowBuilder};
// ADDED: StatusBar
pub use crate::widgets::statusbar::{StatusBar, StatusBarBuilder};
// ADDED: ToolBar
pub use crate::widgets::toolbar::ToolBar;
// ADDED: ListCtrl
pub use crate::widgets::list_ctrl::{ListCtrl, ListCtrlBuilder};
// ADDED: RadioBox
pub use crate::widgets::radiobox::RadioBox;
// ADDED: BitmapComboBox
pub use crate::widgets::bitmapcombobox::{BitmapComboBox, BitmapComboBoxBuilder};
// ADDED: CommandLinkButton
pub use crate::widgets::command_link_button::{CommandLinkButton, CommandLinkButtonBuilder};

// --- ADDED: Menus ---
pub use crate::menus::{ItemKind, Menu, MenuBar, MenuItem};
// REMOVED: No longer exporting ITEM_* constants directly
// pub use crate::menus::menuitem::{ITEM_NORMAL, ITEM_SEPARATOR, ITEM_CHECK, ITEM_RADIO};
// ADDED: Export standard IDs
pub use crate::menus::menuitem::{ID_ABOUT, ID_EXIT, ID_SEPARATOR};
// ADDED: Bitmap
pub use crate::bitmap::Bitmap;

// Replace old ArtProvider const exports with new Enum exports
// pub use crate::art_provider::{
//     ArtProvider, ART_ADD_BOOKMARK, ART_BUTTON, ... (all old consts listed here) ...
// };
pub use crate::art_provider::{ArtClient, ArtId, ArtProvider};

// --- Widget Style Constants ---
// Panel
// pub use crate::widgets::panel::TAB_TRAVERSAL;
// TextCtrl
// pub use crate::widgets::textctrl::TE_PROCESS_ENTER;

// ListBox / CheckListBox
// pub use crate::widgets::listbox::{LB_ALWAYS_SB, LB_HSCROLL, LB_SINGLE, LB_SORT};

// Choice / ComboBox
// pub use crate::widgets::choice::CB_SORT;

// TreeCtrl
pub use crate::widgets::treectrl::{
    TR_DEFAULT_STYLE, TR_EDIT_LABELS, TR_HAS_BUTTONS, TR_HIDE_ROOT, TR_LINES_AT_ROOT, TR_SINGLE,
};
// Gauge
// pub use crate::widgets::gauge::{GA_HORIZONTAL, GA_SMOOTH, GA_VERTICAL};

// ADDED: Slider
// pub use crate::widgets::slider::{
// SL_BOTH, SL_HORIZONTAL, SL_LABELS, SL_MIN_MAX_LABELS, SL_VALUE_LABEL, SL_VERTICAL,
// };

// ADDED: SpinCtrl
// pub use crate::widgets::spinctrl::{SP_ARROW_KEYS, SP_HORIZONTAL, SP_VERTICAL, SP_WRAP};

// Re-export StaticLine - remove old consts
pub use crate::widgets::static_line::{StaticLine, StaticLineBuilder};
// Re-export ScrollBar - remove old consts
pub use crate::widgets::scrollbar::{ScrollBar, ScrollBarBuilder};

// ADDED: Re-export ColourPickerCtrl and related items
pub use crate::widgets::colourpickerctrl::{
    colours, Colour, ColourPickerCtrl, ColourPickerCtrlBuilder,
};

// ADDED: Re-export DatePickerCtrl, DatePickerCtrlBuilder, and DateTime
pub use crate::widgets::datepickerctrl::{DatePickerCtrl, DatePickerCtrlBuilder, DateTime};

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

// ADDED: Font
pub use crate::font::{
    Font, FONTFAMILY_DECORATIVE, FONTFAMILY_DEFAULT, FONTFAMILY_MODERN, FONTFAMILY_ROMAN,
    FONTFAMILY_SCRIPT, FONTFAMILY_SWISS, FONTFAMILY_TELETYPE, FONTSTYLE_ITALIC, FONTSTYLE_NORMAL,
    FONTSTYLE_SLANT, FONTWEIGHT_BOLD, FONTWEIGHT_LIGHT, FONTWEIGHT_NORMAL,
};
pub use crate::font_data::FontData;

// ADDED: ProgressDialog
pub use crate::dialogs::progress_dialog::ProgressDialog;

// ADDED: FilePickerCtrl, its builder, and style constants
pub use crate::widgets::{
    DirPickerCtrl, DirPickerCtrlBuilder, DIRP_CHANGE_DIR, DIRP_DEFAULT_STYLE, DIRP_DIR_MUST_EXIST,
    DIRP_USE_TEXTCTRL,
};
pub use crate::widgets::{
    FilePickerCtrl, FilePickerCtrlBuilder, FLP_CHANGE_DIR, FLP_DEFAULT_STYLE, FLP_FILE_MUST_EXIST,
    FLP_OPEN, FLP_OVERWRITE_PROMPT, FLP_SAVE, FLP_USE_TEXTCTRL,
};
pub use crate::widgets::{
    FontPickerCtrl, FontPickerCtrlBuilder, FNTP_DEFAULT_STYLE, FNTP_FONTDESC_AS_LABEL,
    FNTP_USEFONT_FOR_LABEL, FNTP_USE_TEXTCTRL,
};

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

// --- ADDED: ListCtrl Styles and Constants (already exported at widgets::list_ctrl level, re-exporting from prelude for ease of use)
// These are already exported individually above from `crate::widgets::list_ctrl::*`
// No, they are exported from `crate::widgets::*` which then get them from `crate::widgets::list_ctrl::*`
// The constants from list_ctrl are directly available via `crate::widgets::list_ctrl::LC_LIST` etc.
// The `pub use widgets::list_ctrl::{...}` lines earlier make them available as `crate::LC_LIST` etc.
// So, in prelude, we can just re-export them: `pub use crate::{LC_LIST, LC_REPORT, ...};`
pub use crate::{
    LC_ALIGN_LEFT, LC_ALIGN_TOP, LC_AUTOARRANGE, LC_EDIT_LABELS, LC_HRULES, LC_ICON, LC_LIST,
    LC_NO_HEADER, LC_REPORT, LC_SINGLE_SEL, LC_SMALL_ICON, LC_SORT_ASCENDING, LC_SORT_DESCENDING,
    LC_VRULES,
};
pub use crate::{LIST_FORMAT_CENTRE, LIST_FORMAT_LEFT, LIST_FORMAT_RIGHT};
pub use crate::{LIST_NEXT_ABOVE, LIST_NEXT_ALL, LIST_NEXT_BELOW, LIST_NEXT_LEFT, LIST_NEXT_RIGHT};
pub use crate::{
    LIST_STATE_DISABLED, LIST_STATE_DROPHILITED, LIST_STATE_FOCUSED, LIST_STATE_SELECTED,
};

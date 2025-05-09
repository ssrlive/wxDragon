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
    ALIGN_CENTER_HORIZONTAL, ALIGN_CENTER_VERTICAL, ALIGN_LEFT, ALIGN_RIGHT, ALL,
    BORDER_DEFAULT, BORDER_SIMPLE, BOTTOM, EXPAND, FIXED_MINSIZE, HORIZONTAL, LEFT, RIGHT,
    SHAPED, TOP, VERTICAL,
};
// ADDED: StaticBoxSizer
pub use crate::sizers::staticbox_sizer::{StaticBoxSizer, StaticBoxSizerBuilder};
// ADDED: FlexGridSizer
pub use crate::sizers::flex_grid_sizer::{FlexGridSizer, FlexGridSizerBuilder};

// --- Widgets & Builders ---
pub use crate::widgets::{BitmapButton, BitmapButtonBuilder};
pub use crate::widgets::{Button, ButtonBuilder};
pub use crate::widgets::{CheckBox, CheckBoxBuilder};
pub use crate::widgets::{CheckListBox, CheckListBoxBuilder};
pub use crate::widgets::{Choice, ChoiceBuilder};
pub use crate::widgets::{ComboBox, ComboBoxBuilder};
pub use crate::widgets::{Frame, FrameBuilder};
pub use crate::widgets::{Gauge, GaugeBuilder};
pub use crate::widgets::{ListBox, ListBoxBuilder};
pub use crate::widgets::{Panel, PanelBuilder};
pub use crate::widgets::{RadioButton, RadioButtonBuilder};
pub use crate::widgets::{StaticBox, StaticBoxBuilder};
pub use crate::widgets::{StaticText, StaticTextBuilder};
pub use crate::widgets::{TextCtrl, TextCtrlBuilder};
pub use crate::widgets::{ToggleButton, ToggleButtonBuilder};
pub use crate::widgets::{TreeCtrl, TreeCtrlBuilder};
// ADDED: Slider
pub use crate::widgets::{Slider, SliderBuilder};
// ADDED: SpinCtrl
pub use crate::widgets::{SpinCtrl, SpinCtrlBuilder};
// ADDED: SpinButton
pub use crate::widgets::{SpinButton, SpinButtonBuilder};
// ADDED: Notebook
pub use crate::widgets::{Notebook, NotebookBuilder};
// ADDED: SplitterWindow
pub use crate::widgets::{SplitterWindow, SplitterWindowBuilder};
// ADDED: ScrolledWindow
pub use crate::widgets::{ScrolledWindow, ScrolledWindowBuilder};
// ADDED: StatusBar
pub use crate::widgets::{StatusBar, StatusBarBuilder};
// ADDED: ToolBar
pub use crate::widgets::ToolBar;
// ADDED: ListCtrl
pub use crate::widgets::list_ctrl::{ListCtrl, ListCtrlBuilder};
// ADDED: RadioBox
pub use crate::widgets::RadioBox;
// ADDED: BitmapComboBox
pub use crate::widgets::bitmapcombobox::{BitmapComboBox, BitmapComboBoxBuilder};
// ADDED: CommandLinkButton
pub use crate::widgets::{CommandLinkButton, CommandLinkButtonBuilder};

// --- ADDED: Menus ---
pub use crate::menus::{ItemKind, Menu, MenuBar, MenuItem};
// REMOVED: No longer exporting ITEM_* constants directly
// pub use crate::menus::menuitem::{ITEM_NORMAL, ITEM_SEPARATOR, ITEM_CHECK, ITEM_RADIO};
// ADDED: Export standard IDs
pub use crate::menus::menuitem::{ID_ABOUT, ID_EXIT, ID_SEPARATOR};
// ADDED: Bitmap
pub use crate::bitmap::Bitmap;

// ADDED: ArtProvider and its constants
pub use crate::art_provider::{
    ArtProvider, ART_ADD_BOOKMARK, ART_BUTTON, ART_CDROM, ART_COPY, ART_CROSS_MARK, ART_CUT,
    ART_DELETE, ART_DEL_BOOKMARK, ART_DIALOG, ART_EDIT, ART_ERROR, ART_EXECUTABLE_FILE,
    ART_FILE_OPEN, ART_FILE_SAVE, ART_FILE_SAVE_AS, ART_FIND, ART_FIND_AND_REPLACE, ART_FLOPPY,
    ART_FOLDER, ART_FOLDER_OPEN, ART_FRAME_ICON, ART_GOTO_FIRST, ART_GOTO_LAST, ART_GO_BACK,
    ART_GO_DIR_UP, ART_GO_DOWN, ART_GO_FORWARD, ART_GO_HOME, ART_GO_TO_PARENT, ART_GO_UP,
    ART_HARDDISK, ART_HELP, ART_HELP_BOOK, ART_HELP_FOLDER, ART_HELP_PAGE, ART_HELP_SETTINGS,
    ART_HELP_SIDE_PANEL, ART_INFORMATION, ART_LIST_VIEW, ART_MENU, ART_MESSAGE_BOX,
    ART_MISSING_IMAGE, ART_NEW, ART_NEW_DIR, ART_NORMAL_FILE, ART_OTHER, ART_PASTE, ART_PRINT,
    ART_QUESTION, ART_QUIT, ART_REDO, ART_REMOVABLE, ART_REPORT_VIEW, ART_TICK_MARK, ART_TIP,
    ART_TOOLBAR, ART_UNDO, ART_WARNING,
};

// --- Widget Style Constants ---
// Panel
pub use crate::widgets::panel::TAB_TRAVERSAL;
// TextCtrl
pub use crate::widgets::textctrl::TE_PROCESS_ENTER;
// ListBox / CheckListBox
pub use crate::widgets::listbox::{LB_ALWAYS_SB, LB_HSCROLL, LB_SINGLE, LB_SORT};
// Choice / ComboBox
pub use crate::widgets::choice::CB_SORT;
// TreeCtrl
pub use crate::widgets::treectrl::{
    TR_DEFAULT_STYLE, TR_EDIT_LABELS, TR_HAS_BUTTONS, TR_HIDE_ROOT, TR_LINES_AT_ROOT, TR_SINGLE,
};
// Gauge
pub use crate::widgets::gauge::{GA_HORIZONTAL, GA_SMOOTH, GA_VERTICAL};
// ADDED: Slider
pub use crate::widgets::slider::{
    SL_BOTH, SL_HORIZONTAL, SL_LABELS, SL_MIN_MAX_LABELS, SL_VALUE_LABEL, SL_VERTICAL,
};
// ADDED: SpinCtrl
pub use crate::widgets::spinctrl::{SP_ARROW_KEYS, SP_HORIZONTAL, SP_VERTICAL, SP_WRAP};
// SpinButton uses the same SP_* constants, no need to re-export again unless named differently
// ADDED: Notebook
pub use crate::widgets::notebook::{NB_BOTTOM, NB_DEFAULT, NB_LEFT, NB_RIGHT, NB_TOP};
// ADDED: SplitterWindow (Remove duplicate SP_* exports)
pub use crate::widgets::splitterwindow::{
    /* SP_HORIZONTAL, SP_VERTICAL, */ SP_3D, SP_BORDER, SP_LIVE_UPDATE,
    SP_NOBORDER, /*, SP_DEFAULT_STYLE */
    SP_PERMIT_UNSPLIT,
};
// ADDED: BitmapButton / Button styles
pub use crate::widgets::bitmapbutton::BU_NOTEXT;
pub use crate::widgets::bitmapbutton::{
    BORDER_NONE, BU_BOTTOM, BU_EXACTFIT, BU_LEFT, BU_RIGHT, BU_TOP,
};
// ADDED: ToolBar styles
pub use crate::widgets::toolbar::{
    TB_DOCKABLE, TB_FLAT, TB_HORIZONTAL, TB_NODIVIDER, TB_NOICONS, TB_TEXT, TB_VERTICAL,
};
// ADDED: RadioBox Styles
pub use crate::widgets::radiobox::RA_SPECIFY_COLS; // Add RA_SPECIFY_ROWS later if needed

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
    LC_NO_HEADER, LC_REPORT, LC_SINGLE_SEL, LC_SMALL_ICON, LC_SORT_ASCENDING,
    LC_SORT_DESCENDING, LC_VRULES,
};
pub use crate::{LIST_FORMAT_CENTRE, LIST_FORMAT_LEFT, LIST_FORMAT_RIGHT};
pub use crate::{
    LIST_NEXT_ABOVE, LIST_NEXT_ALL, LIST_NEXT_BELOW, LIST_NEXT_LEFT, LIST_NEXT_RIGHT,
};
pub use crate::{
    LIST_STATE_DISABLED, LIST_STATE_DROPHILITED, LIST_STATE_FOCUSED, LIST_STATE_SELECTED,
};

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

// ADDED: Re-export ActivityIndicator
pub use crate::widgets::activity_indicator::{ActivityIndicator, ActivityIndicatorBuilder};

// ADDED: Re-export SpinCtrlDouble
pub use crate::widgets::spinctrldouble::{SpinCtrlDouble, SpinCtrlDoubleBuilder};

// ADDED: Re-export CalendarCtrl
pub use crate::widgets::calendar_ctrl::{CalendarCtrl, CalendarCtrlBuilder};

// ADDED: Re-export StaticBitmap
pub use crate::widgets::static_bitmap::{StaticBitmap, StaticBitmapBuilder};

// ADDED: Re-export StaticLine
pub use crate::widgets::static_line::{
    StaticLine, StaticLineBuilder, LI_HORIZONTAL, LI_VERTICAL,
};

// ADDED: Re-export ScrollBar
pub use crate::widgets::scrollbar::{ScrollBar, ScrollBarBuilder, SB_HORIZONTAL, SB_VERTICAL};

// ADDED: Dialog and MessageDialog
pub use crate::dialogs::message_dialog::{
    MessageDialog, MessageDialogBuilder, CANCEL, ICON_ERROR, ICON_EXCLAMATION, ICON_HAND,
    ICON_INFORMATION, ICON_QUESTION, ICON_WARNING, NO, OK, YES, CENTRE
};
pub use crate::dialogs::Dialog;
pub use crate::dialogs::text_entry_dialog::{TextEntryDialog, TextEntryDialogBuilder};
pub use crate::dialogs::colour_dialog::ColourDialog;
pub use crate::dialogs::font_dialog::FontDialog;

// ADDED: Font
pub use crate::font::{
    Font, FONTFAMILY_DEFAULT, FONTFAMILY_DECORATIVE, FONTFAMILY_ROMAN, FONTFAMILY_SCRIPT,
    FONTFAMILY_SWISS, FONTFAMILY_MODERN, FONTFAMILY_TELETYPE, FONTSTYLE_NORMAL,
    FONTSTYLE_ITALIC, FONTSTYLE_SLANT, FONTWEIGHT_NORMAL, FONTWEIGHT_LIGHT, FONTWEIGHT_BOLD
};
pub use crate::font_data::FontData;

// ADDED: ProgressDialog
pub use crate::dialogs::progress_dialog::ProgressDialog; 
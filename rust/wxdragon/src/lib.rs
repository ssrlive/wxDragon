// Re-export items from the sys crate for convenience?
// pub use wxdragon_sys as ffi;

pub mod app;
pub mod art_provider;
pub mod base;
pub mod bitmap;
pub mod datetime;
pub mod defs;
pub mod dialogs;
pub mod event;
pub mod font;
pub mod font_data;
pub mod id;
pub mod menus;
pub mod prelude;
pub mod sizers;
pub mod widgets;
pub mod window;

// Re-export the main entry point and handle
pub use app::main;
pub use app::WxdAppHandle;

// Re-export core types
// pub use app::{App, WxApp}; // REMOVED - Not exported from app.rs
pub use datetime::DateTime;
pub use event::{Event, EventType, WxEvtHandler};
pub use sizers::staticbox_sizer::StaticBoxSizer;
pub use sizers::{BoxSizer, Orientation, Sizer, SizerFlags};
pub use widgets::checklistbox::{CheckListBox, CheckListBoxBuilder};
pub use widgets::togglebutton::{ToggleButton, ToggleButtonBuilder};
pub use widgets::{Button, ButtonBuilder};
pub use widgets::{CheckBox, CheckBoxBuilder};
pub use widgets::{Choice, ChoiceBuilder};
pub use widgets::{ComboBox, ComboBoxBuilder};
pub use widgets::{CommandLinkButton, CommandLinkButtonBuilder};
pub use widgets::{Frame, FrameBuilder};
pub use widgets::{ListBox, ListBoxBuilder};
pub use widgets::{Panel, PanelBuilder};
pub use widgets::{RadioButton, RadioButtonBuilder};
pub use widgets::{StaticText, StaticTextBuilder};
pub use widgets::{TextCtrl, TextCtrlBuilder};
pub use window::{Window, WindowUserData, WxWidget};
// ADDED: Re-export TreeCtrl
pub use widgets::treectrl::{TreeCtrl, TreeCtrlBuilder};
pub use widgets::{StaticBox, StaticBoxBuilder};
// ADDED: Re-export Gauge
pub use widgets::gauge::{Gauge, GaugeBuilder};
// ADDED: Re-export Slider
pub use widgets::slider::{Slider, SliderBuilder};
// ADDED: Re-export SpinCtrl
pub use widgets::spinctrl::{SpinCtrl, SpinCtrlBuilder};
// ADDED: Re-export SpinButton
pub use widgets::spinbutton::{SpinButton, SpinButtonBuilder};
// ADDED: Re-export Notebook
pub use widgets::notebook::{Notebook, NotebookBuilder};
// ADDED: Re-export SplitterWindow
pub use widgets::splitterwindow::{SplitterWindow, SplitterWindowBuilder};
// ADDED: Re-export BitmapButton
pub use widgets::bitmapbutton::{BitmapButton, BitmapButtonBuilder};
// ADDED: Re-export ScrolledWindow
pub use widgets::scrolled_window::{ScrolledWindow, ScrolledWindowBuilder};
// ADDED: Re-export StatusBar
pub use widgets::statusbar::{StatusBar, StatusBarBuilder};
// ADDED: Re-export ToolBar
pub use widgets::toolbar::ToolBar;
// ADDED: Re-export Bitmap
pub use bitmap::Bitmap;
// ADDED: Re-export ListCtrl
pub use crate::widgets::list_ctrl::{ListCtrl, ListCtrlBuilder};
pub use crate::widgets::list_ctrl::{
    LC_ALIGN_LEFT, LC_ALIGN_TOP, LC_AUTOARRANGE, LC_EDIT_LABELS, LC_HRULES, LC_ICON, LC_LIST,
    LC_NO_HEADER, LC_REPORT, LC_SINGLE_SEL, LC_SMALL_ICON, LC_SORT_ASCENDING, LC_SORT_DESCENDING,
    LC_VRULES,
};
pub use crate::widgets::list_ctrl::{LIST_FORMAT_CENTRE, LIST_FORMAT_LEFT, LIST_FORMAT_RIGHT};
pub use crate::widgets::list_ctrl::{
    LIST_NEXT_ABOVE, LIST_NEXT_ALL, LIST_NEXT_BELOW, LIST_NEXT_LEFT, LIST_NEXT_RIGHT,
};
pub use crate::widgets::list_ctrl::{
    LIST_STATE_DISABLED, LIST_STATE_DROPHILITED, LIST_STATE_FOCUSED, LIST_STATE_SELECTED,
};

// --- ADDED: Menus Module ---
pub use menus::{ItemKind, Menu, MenuBar, MenuItem};

// ADDED: Re-export ArtProvider and its constants
pub use art_provider::{
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

// --- ADDED: Dialogs ---
pub use dialogs::message_dialog::{MessageDialog, MessageDialogBuilder};
pub use dialogs::Dialog;

// ADDED: Re-export Treebook
pub use widgets::treebook::{Treebook, TreebookBuilder};

// ADDED: Re-export SearchCtrl
pub use widgets::search_ctrl::{SearchCtrl, SearchCtrlBuilder};

// ADDED: Re-export HyperlinkCtrl
pub use widgets::hyperlink_ctrl::{HyperlinkCtrl, HyperlinkCtrlBuilder};

// ADDED: Re-export ActivityIndicator
pub use widgets::activity_indicator::{ActivityIndicator, ActivityIndicatorBuilder};

// ADDED: Re-export SpinCtrlDouble
pub use widgets::spinctrldouble::{SpinCtrlDouble, SpinCtrlDoubleBuilder};

// ADDED: Re-export CalendarCtrl
pub use widgets::calendar_ctrl::{CalendarCtrl, CalendarCtrlBuilder};

// ADDED: Re-export StaticBitmap
pub use widgets::static_bitmap::{StaticBitmap, StaticBitmapBuilder};

// ADDED: Re-export StaticLine
pub use widgets::static_line::{StaticLine, StaticLineBuilder, LI_HORIZONTAL, LI_VERTICAL};

// ADDED: Re-export ScrollBar
pub use widgets::scrollbar::{ScrollBar, ScrollBarBuilder, SB_HORIZONTAL, SB_VERTICAL};

// Dialogs exports
pub use dialogs::{colour_dialog::ColourDialog, font_dialog::FontDialog};

// Font exports
pub use font::Font;
pub use font_data::FontData;

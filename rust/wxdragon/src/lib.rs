// Re-export items from the sys crate for convenience?
// pub use wxdragon_sys as ffi;

#[macro_use]
pub mod macros;
pub mod app;
pub mod art_provider;
pub mod bitmap;
pub mod color;
pub mod datetime;
pub mod dc;
pub mod dialogs;
pub mod dnd;
pub mod event;
pub mod font;
pub mod font_data;
pub mod geometry;
pub mod id;
pub mod menus;
pub mod prelude;
pub mod sizers;
pub mod types;
pub mod utils;
pub mod widgets;
pub mod window;

// Re-export common utilities
pub use utils::WxdArrayString;

// Re-export the main entry point and handle
pub use app::main;
pub use app::set_top_window;
// Re-export call_after function for async support
pub use app::call_after;

// Re-export common types from their new module locations
pub use color::{colours, Colour};
pub use geometry::{Point, Rect, Size, DEFAULT_POSITION, DEFAULT_SIZE};
pub use id::{Id, ID_ANY, ID_HIGHEST};
pub use types::Style;

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
pub use window::{Window, WxWidget};
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
pub use crate::widgets::list_ctrl::{
    ListCtrl, ListCtrlBuilder, ListCtrlStyle, ListItemState, ListNextItemFlag,
};

// ADDED: Re-export ItemData functionality
pub use crate::widgets::item_data::{ItemData, HasItemData};

// --- ADDED: Menus Module ---
pub use menus::{ItemKind, Menu, MenuBar, MenuItem};

// ADDED: Re-export ArtProvider and its constants
pub use art_provider::{ArtClient, ArtId, ArtProvider};

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
pub use widgets::static_line::{StaticLine, StaticLineBuilder, StaticLineStyle};

// ADDED: Re-export ScrollBar
pub use widgets::scrollbar::{ScrollBar, ScrollBarBuilder, ScrollBarStyle};

// Dialogs exports
pub use dialogs::{colour_dialog::ColourDialog, font_dialog::FontDialog};

// Font exports
pub use font::Font;
pub use font_data::FontData;

pub use sizers::flex_grid_sizer::{FlexGridSizer, FlexGridSizerBuilder};

// Re-export drag and drop functionality
pub use dnd::{
    DataObject, DragResult, DropSource, FileDataObject, FileDropTarget, TextDataObject,
    TextDropTarget,
};

// Re-export Device Context functionality
pub use dc::{
    BackgroundMode, BrushStyle, ClientDC, DeviceContext, MemoryDC, PaintDC, PenStyle, ScreenDC,
    WindowDC,
};

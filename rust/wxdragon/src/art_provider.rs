//!
//! Provides access to wxArtProvider for stock art (icons, bitmaps).

use crate::bitmap::Bitmap;
use crate::geometry::Size;
use std::ffi::CString;
use wxdragon_sys as ffi;

// --- ArtId Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArtId {
    Error,
    Information,
    Warning,
    Question,
    HelpSidePanel,
    HelpSettings,
    HelpBook,
    HelpFolder,
    HelpPage,
    AddBookmark,
    DeleteBookmark,
    GoBack,
    GoForward,
    GoUp,
    GoDown,
    GoToParent,
    GoHome,
    GoToFirst,
    GoToLast,
    FileOpen,
    FileSave,
    FileSaveAs,
    Print,
    Help,
    Tip,
    ReportView,
    ListView,
    NewDir,
    HardDisk,
    Floppy,
    CDRom,
    Removable,
    Folder,
    FolderOpen,
    GoDirUp,
    ExecutableFile,
    NormalFile,
    TickMark,
    CrossMark,
    MissingImage,
    New,
    Edit,
    Undo,
    Redo,
    Delete,
    Copy,
    Cut,
    Paste,
    Find,
    FindAndReplace,
    Quit,
    // Add any other specific IDs if they appear in ffi or wxWidgets docs
}

impl ArtId {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtId::Error => "wxART_ERROR",
            ArtId::Information => "wxART_INFORMATION",
            ArtId::Warning => "wxART_WARNING",
            ArtId::Question => "wxART_QUESTION",
            ArtId::HelpSidePanel => "wxART_HELP_SIDE_PANEL",
            ArtId::HelpSettings => "wxART_HELP_SETTINGS",
            ArtId::HelpBook => "wxART_HELP_BOOK",
            ArtId::HelpFolder => "wxART_HELP_FOLDER",
            ArtId::HelpPage => "wxART_HELP_PAGE",
            ArtId::AddBookmark => "wxART_ADD_BOOKMARK",
            ArtId::DeleteBookmark => "wxART_DEL_BOOKMARK",
            ArtId::GoBack => "wxART_GO_BACK",
            ArtId::GoForward => "wxART_GO_FORWARD",
            ArtId::GoUp => "wxART_GO_UP",
            ArtId::GoDown => "wxART_GO_DOWN",
            ArtId::GoToParent => "wxART_GO_TO_PARENT",
            ArtId::GoHome => "wxART_GO_HOME",
            ArtId::GoToFirst => "wxART_GOTO_FIRST",
            ArtId::GoToLast => "wxART_GOTO_LAST",
            ArtId::FileOpen => "wxART_FILE_OPEN",
            ArtId::FileSave => "wxART_FILE_SAVE",
            ArtId::FileSaveAs => "wxART_FILE_SAVE_AS",
            ArtId::Print => "wxART_PRINT",
            ArtId::Help => "wxART_HELP",
            ArtId::Tip => "wxART_TIP",
            ArtId::ReportView => "wxART_REPORT_VIEW",
            ArtId::ListView => "wxART_LIST_VIEW",
            ArtId::NewDir => "wxART_NEW_DIR",
            ArtId::HardDisk => "wxART_HARDDISK",
            ArtId::Floppy => "wxART_FLOPPY",
            ArtId::CDRom => "wxART_CDROM",
            ArtId::Removable => "wxART_REMOVABLE",
            ArtId::Folder => "wxART_FOLDER",
            ArtId::FolderOpen => "wxART_FOLDER_OPEN",
            ArtId::GoDirUp => "wxART_GO_DIR_UP",
            ArtId::ExecutableFile => "wxART_EXECUTABLE_FILE",
            ArtId::NormalFile => "wxART_NORMAL_FILE",
            ArtId::TickMark => "wxART_TICK_MARK",
            ArtId::CrossMark => "wxART_CROSS_MARK",
            ArtId::MissingImage => "wxART_MISSING_IMAGE",
            ArtId::New => "wxART_NEW",
            ArtId::Edit => "wxART_EDIT",
            ArtId::Undo => "wxART_UNDO",
            ArtId::Redo => "wxART_REDO",
            ArtId::Delete => "wxART_DELETE",
            ArtId::Copy => "wxART_COPY",
            ArtId::Cut => "wxART_CUT",
            ArtId::Paste => "wxART_PASTE",
            ArtId::Find => "wxART_FIND",
            ArtId::FindAndReplace => "wxART_FIND_AND_REPLACE",
            ArtId::Quit => "wxART_QUIT",
        }
    }
}

// --- ArtClient Enum ---
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArtClient {
    Button,
    Menu,
    Toolbar,
    FrameIcon,
    MessageBox,
    Dialog,
    Other, // Represents wxART_OTHER, typically an empty string
}

impl ArtClient {
    pub fn as_str(&self) -> &'static str {
        match self {
            ArtClient::Button => "wxART_BUTTON_C",
            ArtClient::Menu => "wxART_MENU_C",
            ArtClient::Toolbar => "wxART_TOOLBAR_C",
            ArtClient::FrameIcon => "wxART_FRAME_ICON_C",
            ArtClient::MessageBox => "wxART_MESSAGE_BOX_C",
            ArtClient::Dialog => "wxART_DIALOG_C",
            ArtClient::Other => "", // Empty string for wxART_OTHER
        }
    }
}

/// Provides static methods to access stock art (bitmaps/icons).
pub struct ArtProvider;

impl ArtProvider {
    /// Retrieves a stock bitmap.
    ///
    /// # Arguments
    /// * `id` - The art ID (e.g., `ART_FILE_OPEN`, `ART_ERROR`). These are string constants.
    /// * `client` - The art client ID (e.g., `ART_BUTTON`, `ART_MENU`). These are string constants.
    ///              An empty string can be used for `wxART_OTHER`.
    /// * `size` - Optional desired size. If `None`, default size is requested (`-1, -1`).
    ///
    /// Returns `Option<Bitmap>`. The bitmap is `None` if not found or an error occurs.
    /// The caller owns the returned `Bitmap` and is responsible for its `Drop`.
    pub fn get_bitmap(id: ArtId, client: ArtClient, size: Option<Size>) -> Option<Bitmap> {
        let c_id = match CString::new(id.as_str()) {
            Ok(s) => s,
            Err(_) => return None, // Should not happen with enum if as_str is correct
        };
        let c_client = match CString::new(client.as_str()) {
            Ok(s) => s,
            Err(_) => return None, // Should not happen with enum
        };

        let ffi_size = size.map_or_else(
            || ffi::wxd_Size {
                width: -1,
                height: -1,
            },
            |s| s.into(),
        );

        let bitmap_ptr =
            unsafe { ffi::wxd_ArtProvider_GetBitmap(c_id.as_ptr(), c_client.as_ptr(), ffi_size) };

        if bitmap_ptr.is_null() {
            None
        } else {
            // The bitmap is created by wxWidgets and ownership is transferred to Rust
            Some(Bitmap::from_ptr_owned(bitmap_ptr))
        }
    }
}

// All old pub const ART_XXX definitions below this point should be removed.
// The edit tool started this process but didn't complete it.
// The following is a placeholder to indicate all subsequent lines of old constants are gone.

// End of file (effectively)

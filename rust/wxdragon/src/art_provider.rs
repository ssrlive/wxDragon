//!
//! Provides access to wxArtProvider for stock art (icons, bitmaps).

use crate::base::Size;
use crate::bitmap::Bitmap;
use std::ffi::CString;
use wxdragon_sys as ffi;

/// Provides static methods to access stock art (bitmaps/icons).
/// Corresponds to `wxArtProvider`.
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
    pub fn get_bitmap(id: &str, client: &str, size: Option<Size>) -> Option<Bitmap> {
        let c_id = match CString::new(id) {
            Ok(s) => s,
            Err(_) => return None, // Invalid CString for id
        };
        let c_client = match CString::new(client) {
            Ok(s) => s,
            Err(_) => return None, // Invalid CString for client
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
            // Safety: pointer is from our FFI, valid if not null.
            // Bitmap::from_ptr is not public, so we construct it directly.
            // This assumes Bitmap(ptr) is the correct way to take ownership.
            Some(Bitmap(bitmap_ptr))
        }
    }
}

// --- Standard Art IDs (wxArtID) ---
pub const ART_ERROR: &str = "wxART_ERROR";
pub const ART_INFORMATION: &str = "wxART_INFORMATION";
pub const ART_WARNING: &str = "wxART_WARNING";
pub const ART_QUESTION: &str = "wxART_QUESTION";

pub const ART_HELP_SIDE_PANEL: &str = "wxART_HELP_SIDE_PANEL";
pub const ART_HELP_SETTINGS: &str = "wxART_HELP_SETTINGS";
pub const ART_HELP_BOOK: &str = "wxART_HELP_BOOK";
pub const ART_HELP_FOLDER: &str = "wxART_HELP_FOLDER";
pub const ART_HELP_PAGE: &str = "wxART_HELP_PAGE";

pub const ART_ADD_BOOKMARK: &str = "wxART_ADD_BOOKMARK";
pub const ART_DEL_BOOKMARK: &str = "wxART_DEL_BOOKMARK";
pub const ART_GO_BACK: &str = "wxART_GO_BACK";
pub const ART_GO_FORWARD: &str = "wxART_GO_FORWARD";
pub const ART_GO_UP: &str = "wxART_GO_UP";
pub const ART_GO_DOWN: &str = "wxART_GO_DOWN";
pub const ART_GO_TO_PARENT: &str = "wxART_GO_TO_PARENT";
pub const ART_GO_HOME: &str = "wxART_GO_HOME";
pub const ART_GOTO_FIRST: &str = "wxART_GOTO_FIRST";
pub const ART_GOTO_LAST: &str = "wxART_GOTO_LAST";

pub const ART_FILE_OPEN: &str = "wxART_FILE_OPEN";
pub const ART_FILE_SAVE: &str = "wxART_FILE_SAVE";
pub const ART_FILE_SAVE_AS: &str = "wxART_FILE_SAVE_AS";
pub const ART_PRINT: &str = "wxART_PRINT";
pub const ART_HELP: &str = "wxART_HELP";
pub const ART_TIP: &str = "wxART_TIP";
pub const ART_REPORT_VIEW: &str = "wxART_REPORT_VIEW";
pub const ART_LIST_VIEW: &str = "wxART_LIST_VIEW";
pub const ART_NEW_DIR: &str = "wxART_NEW_DIR";
pub const ART_HARDDISK: &str = "wxART_HARDDISK";
pub const ART_FLOPPY: &str = "wxART_FLOPPY";
pub const ART_CDROM: &str = "wxART_CDROM";
pub const ART_REMOVABLE: &str = "wxART_REMOVABLE";
pub const ART_FOLDER: &str = "wxART_FOLDER";
pub const ART_FOLDER_OPEN: &str = "wxART_FOLDER_OPEN";
pub const ART_GO_DIR_UP: &str = "wxART_GO_DIR_UP";
pub const ART_EXECUTABLE_FILE: &str = "wxART_EXECUTABLE_FILE";
pub const ART_NORMAL_FILE: &str = "wxART_NORMAL_FILE";
pub const ART_TICK_MARK: &str = "wxART_TICK_MARK";
pub const ART_CROSS_MARK: &str = "wxART_CROSS_MARK";
pub const ART_MISSING_IMAGE: &str = "wxART_MISSING_IMAGE";

pub const ART_NEW: &str = "wxART_NEW";
pub const ART_EDIT: &str = "wxART_EDIT";
pub const ART_UNDO: &str = "wxART_UNDO";
pub const ART_REDO: &str = "wxART_REDO";
pub const ART_DELETE: &str = "wxART_DELETE";
pub const ART_COPY: &str = "wxART_COPY";
pub const ART_CUT: &str = "wxART_CUT";
pub const ART_PASTE: &str = "wxART_PASTE";
pub const ART_FIND: &str = "wxART_FIND";
pub const ART_FIND_AND_REPLACE: &str = "wxART_FIND_AND_REPLACE";
pub const ART_QUIT: &str = "wxART_QUIT";

// --- Standard Art Clients (wxArtClient) ---
pub const ART_BUTTON: &str = "wxART_BUTTON_C";
pub const ART_MENU: &str = "wxART_MENU_C";
pub const ART_TOOLBAR: &str = "wxART_TOOLBAR_C";
pub const ART_FRAME_ICON: &str = "wxART_FRAME_ICON_C";
// wxART_OTHER is typically represented by an empty string or wxART_OTHER_C
// For our API, passing an empty string to client for GetBitmap will work.
pub const ART_OTHER: &str = ""; // Empty string often implies wxART_OTHER or a non-specific client

// wxART_MESSAGE_BOX, wxART_DIALOG are also common clients.
pub const ART_MESSAGE_BOX: &str = "wxART_MESSAGE_BOX_C";
pub const ART_DIALOG: &str = "wxART_DIALOG_C";

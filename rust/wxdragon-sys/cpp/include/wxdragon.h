#ifndef WXDRAGON_H
#define WXDRAGON_H

// Define WXDRAGON_API for export/import
#ifdef _WIN32
    #ifdef WXDRAGON_BUILDING_STATIC_LIB
        #define WXDRAGON_API // Empty for static lib
    #elif defined(WXDRAGON_USING_DLL)
        #define WXDRAGON_API __declspec(dllimport)
    #else // Default to empty, assuming static lib if not specified
        #define WXDRAGON_API
    #endif
#else
    #define WXDRAGON_API // Define as empty for non-Windows (static linking)
#endif

// Include all fundamental types first
#include "wxd_types.h" // Contains all basic C types and opaque struct typedefs

// --- Utility Macros for String Handling (C++ specific) ---
#ifdef __cplusplus
    #include <wx/string.h> // For wxString
    // Helper macro to convert const char* to wxString, handling nulls and UTF-8
    #define WXD_STR_TO_WX_STRING_UTF8_NULL_OK(input_text) wxString::FromUTF8(input_text ? input_text : "")
    // Helper macro for getting wxString result into C buffer
    #define GET_WX_STRING_RESULT(wx_str_expr, c_buffer, c_buf_len) wxd_cpp_utils::copy_wxstring_to_buffer(wx_str_expr, c_buffer, c_buf_len)
#endif
// --- End Utility Macros ---

// Define export macro (used by all sub-headers indirectly via wxd_types.h or if they need it themselves)
#ifndef WXD_EXPORTED
    #define WXD_EXPORTED WXDRAGON_API
#endif

// Main C API block
#ifdef __cplusplus
extern "C" {
#endif

// Include API categories
#include "core/wxd_app.h"
#include "core/wxd_window_base.h"
#include "events/wxd_event_api.h"
#include "widgets/wxd_controls.h"
#include "widgets/wxd_choices.h"
#include "widgets/wxd_containers.h"
#include "widgets/wxd_pickers.h"
#include "widgets/wxd_misc_widgets.h"
#include "sizers/wxd_sizers.h"
#include "dialogs/wxd_dialogs.h"

// DateTime helper functions have been moved into dialogs/wxd_dialogs.h 
// as they were primarily used by DatePickerCtrl.
// If needed more broadly, consider a dedicated core/wxd_datetime_utils.h

#ifdef __cplusplus
} // extern "C"

// C++ specific utility functions or class declarations can go here, outside extern "C"
// These are typically for helpers used by the C++ implementations of the C API.
namespace wxd_cpp_utils {
    // Declaration for the utility function used by GET_WX_STRING_RESULT macro
    // Its definition is in a .cpp file (e.g., wxd_utils.cpp)
    size_t copy_wxstring_to_buffer(const wxString& str, char* buffer, size_t buffer_len);
}

#endif // __cplusplus

#endif // WXDRAGON_H
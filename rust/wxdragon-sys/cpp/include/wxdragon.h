#ifndef WXDRAGON_H
#define WXDRAGON_H

// Define WXDRAGON_API for export/import
        #define WXDRAGON_API

// Define export macro (used by all sub-headers indirectly via wxd_types.h or if they need it themselves)
#ifndef WXD_EXPORTED
    #define WXD_EXPORTED WXDRAGON_API
#endif

// Include all fundamental types first
#include "wxd_types.h" // Contains all basic C types and opaque struct typedefs

#ifdef __cplusplus
// Include C++ specific utility functions and macros
// This should make WXD_STR_TO_WX_STRING_UTF8_NULL_OK and GET_WX_STRING_RESULT available
// as well as the declaration for wxd_cpp_utils::copy_wxstring_to_buffer
#include "../src/wxd_utils.h" 
#endif

// Main C API block
#ifdef __cplusplus
extern "C" {
#endif

// Include API categories
#include "core/wxd_app.h"
#include "core/wxd_window_base.h"
#include "events/wxd_event_api.h"
#include "events/wxd_droptarget_api.h" // Extended drop target callbacks
#include "widgets/wxd_controls.h"
#include "widgets/wxd_choices.h"
#include "widgets/wxd_containers.h"
#include "widgets/wxd_notebook.h"
#include "widgets/wxd_pickers.h"
#include "widgets/wxd_file_ctrl.h"
#include "widgets/wxd_adv_ui.h" // For wxNotificationMessage etc.
#include "widgets/wxd_misc_widgets.h"
#include "widgets/wxd_editablelistbox.h" // For wxEditableListBox
#include "sizers/wxd_sizers.h"
#include "dialogs/wxd_dialogs.h"
#include "widgets/wxd_aui.h"
#include "dnd/wxd_dnd.h" // Drag and drop functionality
#include "graphics/wxd_dc.h" // Device context functionality

#ifdef __cplusplus
} // extern "C"

#endif // __cplusplus

#endif // WXDRAGON_H
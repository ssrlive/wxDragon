#ifndef WXD_WINDOW_BASE_H
#define WXD_WINDOW_BASE_H

#include "../wxd_types.h"

// --- Common Window Functions ---
WXD_EXPORTED void wxd_Window_SetSizer(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED void wxd_Window_SetSizerAndFit(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer);
WXD_EXPORTED int wxd_Window_GetId(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Fit(wxd_Window_t* window);
WXD_EXPORTED wxd_Size wxd_Window_GetBestSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Destroy(wxd_Window_t* window); // Generic destroy
WXD_EXPORTED void wxd_Window_SetBackgroundColor(wxd_Window_t* window, wxd_Colour_t color);
WXD_EXPORTED void wxd_Window_SetMinSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED void wxd_Window_Refresh(wxd_Window_t* window, int eraseBackground, const wxd_Rect* rect);
WXD_EXPORTED void wxd_Window_SetToolTip(wxd_Window_t* window, const char* tipString);

// Declarations for functions that were previously in wxdragon.h directly
WXD_EXPORTED void wxd_Window_Show(wxd_Window_t* self, bool show);
WXD_EXPORTED bool wxd_Window_Close(wxd_Window_t* self, bool force);
WXD_EXPORTED void wxd_Window_SetId(wxd_Window_t* self, int id);
WXD_EXPORTED void wxd_Window_SetLabel(wxd_Window_t* self, const char* label);
WXD_EXPORTED char* wxd_Window_GetLabel(wxd_Window_t* self); // Caller must free with wxd_free_string

// Window cleanup notifier functions
WXD_EXPORTED void wxd_Window_AttachCleanupNotifier(wxd_Window_t* win_ptr);
WXD_EXPORTED void wxd_Window_DetachCleanupNotifier(wxd_Window_t* win_ptr);

#endif // WXD_WINDOW_BASE_H 
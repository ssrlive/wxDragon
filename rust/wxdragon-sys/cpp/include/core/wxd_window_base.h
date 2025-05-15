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

// Size and position related functions
WXD_EXPORTED wxd_Size wxd_Window_GetSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_SetSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED void wxd_Window_SetSizeWithPos(wxd_Window_t* window, int x, int y, int width, int height, int sizeFlags);
WXD_EXPORTED wxd_Size wxd_Window_GetClientSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_SetClientSize(wxd_Window_t* window, wxd_Size size);
WXD_EXPORTED wxd_Size wxd_Window_GetMinSize(wxd_Window_t* window);
WXD_EXPORTED void wxd_Window_Move(wxd_Window_t* window, int x, int y);
WXD_EXPORTED void wxd_Window_Center(wxd_Window_t* window);
WXD_EXPORTED wxd_Point wxd_Window_ClientToScreen(wxd_Window_t* window, wxd_Point pt);
WXD_EXPORTED wxd_Point wxd_Window_ScreenToClient(wxd_Window_t* window, wxd_Point pt);

// Declarations for functions that were previously in wxdragon.h directly
WXD_EXPORTED void wxd_Window_Show(wxd_Window_t* self, bool show);
WXD_EXPORTED bool wxd_Window_Close(wxd_Window_t* self, bool force);
WXD_EXPORTED void wxd_Window_SetId(wxd_Window_t* self, int id);
WXD_EXPORTED void wxd_Window_SetLabel(wxd_Window_t* self, const char* label);
WXD_EXPORTED char* wxd_Window_GetLabel(wxd_Window_t* self); // Caller must free with wxd_free_string

WXD_EXPORTED bool wxd_Window_IsEnabled(wxd_Window_t *self);
WXD_EXPORTED void wxd_Window_Enable(wxd_Window_t *self, bool enable);

WXD_EXPORTED wxd_Window_t* wxd_Window_GetParent(wxd_Window_t* self);
WXD_EXPORTED wxd_Window_t* wxd_Window_GetGrandParent(wxd_Window_t* self);

WXD_EXPORTED void wxd_Window_SetFont(wxd_Window_t* self, const wxd_Font_t* font);
WXD_EXPORTED wxd_Font_t* wxd_Window_GetFont(wxd_Window_t* self);
WXD_EXPORTED wxd_Point wxd_Window_GetPosition(wxd_Window_t* self);

#endif // WXD_WINDOW_BASE_H 
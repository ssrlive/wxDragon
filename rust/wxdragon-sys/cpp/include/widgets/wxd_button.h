#ifndef WXD_BUTTON_H
#define WXD_BUTTON_H

#include "../wxd_types.h"

// --- Button Functions ---
WXD_EXPORTED wxd_Button_t* wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_Button_Destroy(wxd_Button_t* button); // Note: Consider if needed, generic wxd_Window_Destroy might suffice
WXD_EXPORTED void wxd_Button_SetLabel(wxd_Button_t* button, const char* label);
WXD_EXPORTED int wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, int buffer_len);

#endif // WXD_BUTTON_H 
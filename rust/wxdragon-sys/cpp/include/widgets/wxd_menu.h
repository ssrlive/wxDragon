#ifndef WXD_MENU_H
#define WXD_MENU_H

#include "../wxd_types.h"

// --- MenuBar, Menu, MenuItem Functions ---
WXD_EXPORTED wxd_MenuBar_t* wxd_MenuBar_Create(wxd_Style_t style);
WXD_EXPORTED void wxd_MenuBar_Append(wxd_MenuBar_t* menubar, wxd_Menu_t* menu, const char* title);
WXD_EXPORTED wxd_Menu_t* wxd_Menu_Create(const char* title, wxd_Style_t style);
WXD_EXPORTED void wxd_Menu_Destroy(wxd_Menu_t* menu);
WXD_EXPORTED wxd_MenuItem_t* wxd_Menu_Append(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind);
WXD_EXPORTED void wxd_Menu_AppendSeparator(wxd_Menu_t* menu);
WXD_EXPORTED void wxd_MenuItem_Destroy(wxd_MenuItem_t* item);

// --- MenuItem State Functions ---
WXD_EXPORTED void wxd_MenuItem_SetLabel(wxd_MenuItem_t* item, const char* label);
WXD_EXPORTED char* wxd_MenuItem_GetLabel(wxd_MenuItem_t* item); // Caller must free with wxd_free_string
WXD_EXPORTED void wxd_MenuItem_Enable(wxd_MenuItem_t* item, bool enable);
WXD_EXPORTED bool wxd_MenuItem_IsEnabled(wxd_MenuItem_t* item);
WXD_EXPORTED void wxd_MenuItem_Check(wxd_MenuItem_t* item, bool check);
WXD_EXPORTED bool wxd_MenuItem_IsChecked(wxd_MenuItem_t* item);

#endif // WXD_MENU_H 
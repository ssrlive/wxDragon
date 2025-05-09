#ifndef WXD_APP_H
#define WXD_APP_H

#include "../wxd_types.h" // Adjust path as necessary if wxd_types.h is at the root of include/

// --- App Functions ---
WXD_EXPORTED wxd_App_t* wxd_GetApp();
WXD_EXPORTED int wxd_Main(int argc, char** argv, wxd_OnInitCallback on_init, void* userData);
WXD_EXPORTED void wxd_App_SetTopWindow(wxd_App_t* app, wxd_Window_t* window);

// Utility to free strings allocated by wxDragon C API
WXD_EXPORTED void wxd_free_string(char* str);

#endif // WXD_APP_H 
#ifndef WXD_AUI_H
#define WXD_AUI_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

WXD_EXPORTED wxd_AuiMDIParentFrame_t* wxd_AuiMDIParentFrame_Create(wxd_Window_t* parent, int id, const char* title, wxd_Point pos, wxd_Size size, long style, const char* name);

WXD_EXPORTED wxd_AuiMDIChildFrame_t* wxd_AuiMDIChildFrame_Create(wxd_AuiMDIParentFrame_t* parent, int id, const char* title, wxd_Point pos, wxd_Size size, long style, const char* name);

// Potentially other wxAuiMDIChildFrame specific functions will be added here.

// --- wxAuiNotebook ---
WXD_EXPORTED wxd_AuiNotebook_t* wxd_AuiNotebook_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, long style);
WXD_EXPORTED bool wxd_AuiNotebook_AddPage(wxd_AuiNotebook_t* self, wxd_Window_t* page, const char* caption, bool select, int bitmap_id);
WXD_EXPORTED size_t wxd_AuiNotebook_GetPageCount(wxd_AuiNotebook_t* self);
WXD_EXPORTED size_t wxd_AuiNotebook_SetSelection(wxd_AuiNotebook_t* self, size_t new_page);
// WXD_EXPORTED wxd_Window_t* wxd_AuiNotebook_GetPage(wxd_AuiNotebook_t* self, size_t page_idx); // TODO: decide if GetPage is needed and how to manage its lifecycle

// --- wxAuiToolBar ---
WXD_EXPORTED wxd_AuiToolBar_t* wxd_AuiToolBar_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, long style);
WXD_EXPORTED void wxd_AuiToolBar_AddTool(wxd_AuiToolBar_t* self, int tool_id, const char* label, /*wxd_Bitmap_t* bitmap, wxd_Bitmap_t* disabled_bitmap,*/ const char* short_help_string, WXDItemKindCEnum kind);
WXD_EXPORTED void wxd_AuiToolBar_AddLabel(wxd_AuiToolBar_t* self, int tool_id, const char* label, int width);
WXD_EXPORTED void wxd_AuiToolBar_AddControl(wxd_AuiToolBar_t* self, wxd_Control_t* control, const char* label);
WXD_EXPORTED void wxd_AuiToolBar_AddSeparator(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_AddSpacer(wxd_AuiToolBar_t* self, int pixels);
WXD_EXPORTED void wxd_AuiToolBar_AddStretchSpacer(wxd_AuiToolBar_t* self, int proportion);
WXD_EXPORTED void wxd_AuiToolBar_Realize(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_SetToolBitmapSize(wxd_AuiToolBar_t* self, wxd_Size size);
WXD_EXPORTED wxd_Size wxd_AuiToolBar_GetToolBitmapSize(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_SetOverflowVisible(wxd_AuiToolBar_t* self, bool visible);
WXD_EXPORTED bool wxd_AuiToolBar_GetOverflowVisible(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_SetGripperVisible(wxd_AuiToolBar_t* self, bool visible);
WXD_EXPORTED bool wxd_AuiToolBar_GetGripperVisible(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_SetToolDropDown(wxd_AuiToolBar_t* self, int tool_id, bool dropdown);
WXD_EXPORTED bool wxd_AuiToolBar_GetToolDropDown(wxd_AuiToolBar_t* self, int tool_id);
WXD_EXPORTED void wxd_AuiToolBar_EnableTool(wxd_AuiToolBar_t* self, int tool_id, bool enable);
WXD_EXPORTED bool wxd_AuiToolBar_GetToolEnabled(wxd_AuiToolBar_t* self, int tool_id);
WXD_EXPORTED int wxd_AuiToolBar_GetToolCount(wxd_AuiToolBar_t* self);
WXD_EXPORTED void wxd_AuiToolBar_ClearTools(wxd_AuiToolBar_t* self);
WXD_EXPORTED bool wxd_AuiToolBar_DeleteTool(wxd_AuiToolBar_t* self, int tool_id);

#ifdef __cplusplus
}
#endif

#endif // WXD_AUI_H 
#ifndef WXD_MISC_WIDGETS_H
#define WXD_MISC_WIDGETS_H

#include "../wxd_types.h"

// --- Frame ---
WXD_EXPORTED wxd_Frame_t* wxd_Frame_Create(wxd_Window_t* parent, wxd_Id id, const char* title, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_Frame_Destroy(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_Show(wxd_Frame_t* frame, bool show);
WXD_EXPORTED void wxd_Frame_SetTitle(wxd_Frame_t* frame, const char* title);
WXD_EXPORTED void wxd_Frame_Centre(wxd_Frame_t* frame, wxd_Direction_t direction);
WXD_EXPORTED void wxd_Frame_Close(wxd_Frame_t* frame, bool force);
WXD_EXPORTED void wxd_Frame_SetMenuBar(wxd_Frame_t* frame, wxd_MenuBar_t* menubar);
WXD_EXPORTED void wxd_Frame_SetStatusBar(wxd_Frame_t* frame, wxd_StatusBar_t* statusBar);
WXD_EXPORTED void wxd_Frame_SetToolBar(wxd_Frame_t* frame, wxd_ToolBar_t* toolBar);
WXD_EXPORTED wxd_ToolBar_t* wxd_Frame_CreateToolBar(wxd_Frame_t* frame, wxd_Style_t style, wxd_Id id);
WXD_EXPORTED wxd_StatusBar_t* wxd_Frame_CreateStatusBar(wxd_Frame_t* frame, int number, long style, int id, const char* name);
WXD_EXPORTED void wxd_Frame_CenterOnScreen(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_SetStatusText(wxd_Frame_t* frame, const char* text, int number);
WXD_EXPORTED char* wxd_Frame_GetTitle(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_Iconize(wxd_Frame_t* frame, bool iconize);
WXD_EXPORTED bool wxd_Frame_IsIconized(wxd_Frame_t* frame);
WXD_EXPORTED void wxd_Frame_Maximize(wxd_Frame_t* frame, bool maximize);
WXD_EXPORTED bool wxd_Frame_IsMaximized(wxd_Frame_t* frame);

// --- StatusBar ---
WXD_EXPORTED wxd_StatusBar_t* wxd_StatusBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Style_t style);
WXD_EXPORTED void wxd_StatusBar_SetFieldsCount(wxd_StatusBar_t* self, int count);
WXD_EXPORTED void wxd_StatusBar_SetStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);
WXD_EXPORTED void wxd_StatusBar_SetStatusWidths(wxd_StatusBar_t* self, int count, const int* widths);
WXD_EXPORTED void wxd_StatusBar_PushStatusText(wxd_StatusBar_t* self, const char* text, int fieldIndex);
WXD_EXPORTED void wxd_StatusBar_PopStatusText(wxd_StatusBar_t* self, int fieldIndex);

// --- ToolBar ---
WXD_EXPORTED wxd_ToolBar_t* wxd_ToolBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void* wxd_ToolBar_AddTool(wxd_ToolBar_t* self, wxd_Id toolId, const char* label, wxd_Bitmap_t* bitmap, wxd_Bitmap_t* bitmapDisabled, int kind, const char* shortHelp, const char* longHelp);
WXD_EXPORTED void wxd_ToolBar_AddSeparator(wxd_ToolBar_t* self);
WXD_EXPORTED void wxd_ToolBar_AddControl(wxd_ToolBar_t* self, wxd_Window_t* control);
WXD_EXPORTED bool wxd_ToolBar_Realize(wxd_ToolBar_t* self);
WXD_EXPORTED void wxd_ToolBar_EnableTool(wxd_ToolBar_t* self, wxd_Id toolId, bool enable);
WXD_EXPORTED void wxd_ToolBar_ToggleTool(wxd_ToolBar_t* self, wxd_Id toolId, bool toggle);
WXD_EXPORTED bool wxd_ToolBar_IsToolEnabled(wxd_ToolBar_t* self, wxd_Id toolId);
WXD_EXPORTED bool wxd_ToolBar_GetToolState(wxd_ToolBar_t* self, wxd_Id toolId);
WXD_EXPORTED void wxd_ToolBar_SetToolShortHelp(wxd_ToolBar_t* self, wxd_Id toolId, const char* helpString);

// --- MenuBar, Menu, MenuItem ---
WXD_EXPORTED wxd_MenuBar_t* wxd_MenuBar_Create(wxd_Style_t style);
WXD_EXPORTED void wxd_MenuBar_Append(wxd_MenuBar_t* menubar, wxd_Menu_t* menu, const char* title);
WXD_EXPORTED wxd_Menu_t* wxd_Menu_Create(const char* title, wxd_Style_t style);
WXD_EXPORTED void wxd_Menu_Destroy(wxd_Menu_t* menu);
WXD_EXPORTED wxd_MenuItem_t* wxd_Menu_Append(wxd_Menu_t* menu, wxd_Id id, const char* item, const char* helpString, int kind);
WXD_EXPORTED void wxd_Menu_AppendSeparator(wxd_Menu_t* menu);
WXD_EXPORTED void wxd_MenuItem_Destroy(wxd_MenuItem_t* item);

// --- CalendarCtrl ---
WXD_EXPORTED wxd_CalendarCtrl_t* wxd_CalendarCtrl_Create(wxd_Window_t* parent, wxd_Id id, const wxd_DateTime_t* date, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date);
WXD_EXPORTED wxd_DateTime_t wxd_CalendarCtrl_GetDate(wxd_CalendarCtrl_t* self);

// --- ArtProvider ---
WXD_EXPORTED wxd_Bitmap_t* wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size);

// --- Bitmap ---
WXD_EXPORTED wxd_Bitmap_t* wxd_Bitmap_CreateFromRGBA(const unsigned char* data, int width, int height);
WXD_EXPORTED void wxd_Bitmap_Destroy(wxd_Bitmap_t* bitmap);
WXD_EXPORTED int wxd_Bitmap_GetWidth(wxd_Bitmap_t* bitmap);
WXD_EXPORTED int wxd_Bitmap_GetHeight(wxd_Bitmap_t* bitmap);
WXD_EXPORTED bool wxd_Bitmap_IsOk(wxd_Bitmap_t* bitmap);
WXD_EXPORTED wxd_Bitmap_t* wxd_Bitmap_Clone(wxd_Bitmap_t* bitmap);


#endif // WXD_MISC_WIDGETS_H 
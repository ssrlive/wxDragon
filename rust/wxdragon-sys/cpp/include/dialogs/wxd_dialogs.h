#ifndef WXD_DIALOGS_H
#define WXD_DIALOGS_H

#include "../wxd_types.h"

// --- Dialog (Base) ---
WXD_EXPORTED int wxd_Dialog_ShowModal(wxd_Dialog_t* self);

// --- MessageDialog ---
WXD_EXPORTED wxd_MessageDialog_t* wxd_MessageDialog_Create(wxd_Window_t* parent, const char* message, const char* caption, wxd_Style_t style);

// --- FileDialog ---
WXD_EXPORTED wxd_ArrayString_t* wxd_ArrayString_Create(); // Helper for paths/filenames
WXD_EXPORTED void wxd_ArrayString_Free(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetCount(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetString(wxd_ArrayString_t* self, int index, char* buffer, int bufLen);
WXD_EXPORTED bool wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str);
WXD_EXPORTED void wxd_ArrayString_Clear(wxd_ArrayString_t* self);

WXD_EXPORTED wxd_FileDialog_t* wxd_FileDialog_Create(wxd_Window_t* parent, const char* message, const char* defaultDir, const char* defaultFile, const char* wildcard, wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_FileDialog_GetPath(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetPaths(wxd_FileDialog_t* self, wxd_ArrayString_t* paths);
WXD_EXPORTED int wxd_FileDialog_GetFilename(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetFilenames(wxd_FileDialog_t* self, wxd_ArrayString_t* filenames);
WXD_EXPORTED int wxd_FileDialog_GetDirectory(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED int wxd_FileDialog_GetFilterIndex(wxd_FileDialog_t* self);

// --- ColourDialog ---
WXD_EXPORTED wxd_ColourData_t* wxd_ColourData_Create(void);
WXD_EXPORTED void wxd_ColourData_SetColour(wxd_ColourData_t* self, wxd_Colour_t colour);
WXD_EXPORTED wxd_Colour_t wxd_ColourData_GetColour(wxd_ColourData_t* self);
WXD_EXPORTED void wxd_ColourData_Destroy(wxd_ColourData_t* self);
WXD_EXPORTED wxd_ColourDialog_t* wxd_ColourDialog_Create(wxd_Window_t* parent, const char* title, wxd_ColourData_t* data);
WXD_EXPORTED wxd_ColourData_t* wxd_ColourDialog_GetColourData(wxd_ColourDialog_t* self);

// --- FontDialog ---
WXD_EXPORTED wxd_FontData_t* wxd_FontData_Create(void);
WXD_EXPORTED void wxd_FontData_Destroy(wxd_FontData_t* self);
WXD_EXPORTED void wxd_FontData_EnableEffects(wxd_FontData_t* self, bool enable);
WXD_EXPORTED bool wxd_FontData_GetEnableEffects(wxd_FontData_t* self);
WXD_EXPORTED void wxd_FontData_SetInitialFont(wxd_FontData_t* self, const wxd_Font_t* font);

WXD_EXPORTED wxd_Font_t* wxd_Font_Create(void);
WXD_EXPORTED wxd_Font_t* wxd_Font_CreateEx(int point_size, int family, int style, int weight, bool underlined, const char* face_name);
WXD_EXPORTED bool wxd_Font_AddPrivateFont(const char* font_file_path);
WXD_EXPORTED void wxd_Font_Destroy(wxd_Font_t* font);
WXD_EXPORTED int wxd_Font_GetPointSize(wxd_Font_t* self);
WXD_EXPORTED int wxd_Font_GetFamily(wxd_Font_t* self);
WXD_EXPORTED int wxd_Font_GetStyle(wxd_Font_t* self);
WXD_EXPORTED int wxd_Font_GetWeight(wxd_Font_t* self);
WXD_EXPORTED bool wxd_Font_GetUnderlined(wxd_Font_t* self);
WXD_EXPORTED int wxd_Font_GetFaceName(wxd_Font_t* self, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_Font_IsOk(wxd_Font_t* self);

WXD_EXPORTED wxd_FontDialog_t* wxd_FontDialog_Create(wxd_Window_t* parent, const char* title, wxd_FontData_t* data);
WXD_EXPORTED wxd_FontData_t* wxd_FontDialog_GetFontData(wxd_FontDialog_t* self);
WXD_EXPORTED wxd_Font_t* wxd_FontDialog_GetFont(wxd_FontDialog_t* self);

// --- TextEntryDialog ---
WXD_EXPORTED wxd_TextEntryDialog_t* wxd_TextEntryDialog_Create(wxd_Window_t* parent, const char* message, const char* caption, const char* defaultValue, wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_TextEntryDialog_GetValue(wxd_TextEntryDialog_t* self, char* buffer, int bufLen);

// --- ProgressDialog ---
WXD_EXPORTED wxd_ProgressDialog_t* wxd_ProgressDialog_Create(wxd_Window_t* parent, const char* title, const char* message, int maximum, wxd_Style_t style);
WXD_EXPORTED bool wxd_ProgressDialog_Update(wxd_ProgressDialog_t* self, int value, const char* newmsg, bool* skip);
WXD_EXPORTED bool wxd_ProgressDialog_Pulse(wxd_ProgressDialog_t* self, const char* newmsg, bool* skip);
WXD_EXPORTED void wxd_ProgressDialog_Resume(wxd_ProgressDialog_t* self);
WXD_EXPORTED int wxd_ProgressDialog_GetValue(wxd_ProgressDialog_t* self);
WXD_EXPORTED int wxd_ProgressDialog_GetRange(wxd_ProgressDialog_t* self);
WXD_EXPORTED bool wxd_ProgressDialog_WasCancelled(wxd_ProgressDialog_t* self);
WXD_EXPORTED bool wxd_ProgressDialog_WasSkipped(wxd_ProgressDialog_t* self);

// --- DateTime Helper Functions (moved here as they are used by DatePickerCtrl in this file) ---
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Default();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Now();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_FromComponents(int year, unsigned short month, short day, short hour, short minute, short second);
WXD_EXPORTED bool wxd_DateTime_IsValid(const wxd_DateTime_t* dt);

#endif // WXD_DIALOGS_H 
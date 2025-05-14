#ifndef WXD_COMBOBOX_H
#define WXD_COMBOBOX_H

#include "../wxd_types.h"

// --- ComboBox Functions ---
WXD_EXPORTED wxd_ComboBox_t* wxd_ComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item);
WXD_EXPORTED void wxd_ComboBox_Clear(wxd_ComboBox_t* combo);
WXD_EXPORTED int wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo);
WXD_EXPORTED int wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index);
WXD_EXPORTED int wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, int buffer_len);
WXD_EXPORTED unsigned int wxd_ComboBox_GetCount(wxd_ComboBox_t* combo);
WXD_EXPORTED void wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value);
WXD_EXPORTED int wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, int buffer_len);

#endif // WXD_COMBOBOX_H 
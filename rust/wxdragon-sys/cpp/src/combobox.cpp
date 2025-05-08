#include "wx/combobox.h"
#include "wx/window.h"
#include "wx/string.h"
#include "wx/arrstr.h"
#include "wx/defs.h" // For wxNOT_FOUND
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_ComboBox_t* wxd_ComboBox_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const char* value, // Initial value for text field
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin) return nullptr;

    wxString wxValue = wxString::FromUTF8(value ? value : "");
    wxComboBox* combo = new wxComboBox(parentWin, id, wxValue, 
                                     wxd_cpp_utils::to_wx(pos),
                                     wxd_cpp_utils::to_wx(size),
                                     0, nullptr, style);
    return (wxd_ComboBox_t*)combo;
}

WXD_EXPORTED void wxd_ComboBox_Append(wxd_ComboBox_t* combo, const char* item) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb && item) {
        cb->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void wxd_ComboBox_Clear(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->Clear(); // Clears list items
        // cb->SetValue(""); // Optionally clear text field too?
                          // Standard Clear() only clears the list.
    }
}

WXD_EXPORTED int wxd_ComboBox_GetSelection(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return wxNOT_FOUND;
    return cb->GetSelection(); // Returns wxNOT_FOUND (-1) if nothing selected
}

WXD_EXPORTED int wxd_ComboBox_GetStringSelection(wxd_ComboBox_t* combo, char* buffer, int buffer_len) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return 0;

    wxString selection = cb->GetStringSelection();
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
    return (int)(needed_len_no_null + 1);
}

WXD_EXPORTED void wxd_ComboBox_SetSelection(wxd_ComboBox_t* combo, int index) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        // SetSelection also updates the text field to the selected string
        cb->SetSelection(index);
    }
}

WXD_EXPORTED int wxd_ComboBox_GetString(wxd_ComboBox_t* combo, int index, char* buffer, int buffer_len) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return 0;
    if (index < 0 || (unsigned int)index >= cb->GetCount()) {
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return 0;
    }

    wxString item = cb->GetString((unsigned int)index);
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, (size_t)buffer_len);
    return (int)(needed_len_no_null + 1);
}

WXD_EXPORTED unsigned int wxd_ComboBox_GetCount(wxd_ComboBox_t* combo) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return 0;
    return cb->GetCount();
}

WXD_EXPORTED void wxd_ComboBox_SetValue(wxd_ComboBox_t* combo, const char* value) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (cb) {
        cb->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED int wxd_ComboBox_GetValue(wxd_ComboBox_t* combo, char* buffer, int buffer_len) {
    wxComboBox* cb = (wxComboBox*)combo;
    if (!cb) return 0;
    
    wxString value = cb->GetValue();
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
    return (int)(needed_len_no_null + 1);
}

// Destroy handled by parent window

} // extern "C" 
#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/wx.h>
#include <wx/button.h>
#include <string.h> // For strlen, strncpy

// --- Helper Functions (duplicated from frame.cpp for now) ---

inline wxPoint wxd_to_wx(const wxd_Point& p) {
    if (p.x == -1 && p.y == -1) return wxDefaultPosition;
    return wxPoint(p.x, p.y);
}

inline wxSize wxd_to_wx(const wxd_Size& s) {
    if (s.width == -1 && s.height == -1) return wxDefaultSize;
    return wxSize(s.width, s.height);
}

// --- Button Functions Implementation ---

wxd_Button_t* wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style) {
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);

    wxButton* wx_button = new wxButton(wx_parent,
                                     id,
                                     wxString::FromUTF8(label ? label : ""),
                                     wxd_cpp_utils::to_wx(pos),
                                     wxd_cpp_utils::to_wx(size),
                                     style);

    // Event handler will be created lazily on first Bind call.
    return reinterpret_cast<wxd_Button_t*>(wx_button);
}

void wxd_Button_Destroy(wxd_Button_t* button) {
    if (!button) return;
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    // Schedule for destruction. Cleanup happens via wxEVT_DESTROY.
    wx_button->Destroy();
}

void wxd_Button_SetLabel(wxd_Button_t* button, const char* label) {
    if (!button) return;
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    wx_button->SetLabel(wxString::FromUTF8(label ? label : ""));
}

int wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, int buffer_len) {
    if (!button) return 0; // Indicate error
    wxButton* wx_button = reinterpret_cast<wxButton*>(button);
    
    wxString label = wx_button->GetLabel();
    
    // Use the utility function
    // The utility returns size_t (source_len without null), FFI expects int (required_len with null)
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, (size_t)buffer_len);
    return (int)(needed_len_no_null + 1); // Return required size including null terminator for consistency with old logic
}

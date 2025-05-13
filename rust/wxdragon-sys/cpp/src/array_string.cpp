#include "wx/wx.h"
#include "wx/arrstr.h"
#include "../include/wxdragon.h"
#include "wxd_utils.h"

// --- wxd_ArrayString Functions ---

WXD_EXPORTED bool wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str) {
    if (!self || !self->internal_data || !str) return false;
    
    wxArrayString* wx_arr_str = static_cast<wxArrayString*>(self->internal_data);
    wx_arr_str->Add(wxString::FromUTF8(str));
    return true;
}

WXD_EXPORTED void wxd_ArrayString_Clear(wxd_ArrayString_t* self) {
    if (!self || !self->internal_data) return;
    
    wxArrayString* wx_arr_str = static_cast<wxArrayString*>(self->internal_data);
    wx_arr_str->Clear();
} 
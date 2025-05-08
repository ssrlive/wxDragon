#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/wx.h>
#include <wx/stattext.h> // Include wxStaticText header
#include <string> // For std::string conversion

// Helper to convert wxPoint/wxSize if needed (assuming direct use is ok for now)
// inline wxPoint ToWxPoint(wxd_Point p) { return wxPoint(p.x, p.y); }
// inline wxSize ToWxSize(wxd_Size s) { return wxSize(s.width, s.height); }

extern "C" {

// --- StaticText Functions ---

WXD_EXPORTED wxd_StaticText_t* wxd_StaticText_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    const char* label, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style)
{
    if (!parent) {
        // Maybe allow null parent for top-level? Unlikely for StaticText.
        return nullptr; 
    }
    wxWindow* wx_parent = reinterpret_cast<wxWindow*>(parent);
    
    // Ensure label is valid UTF-8 before converting
    wxString wx_label = wxString::FromUTF8(label ? label : ""); 
    
    wxStaticText* stext = new wxStaticText(
        wx_parent, 
        id, 
        wx_label, 
        wxd_cpp_utils::to_wx(pos),
        wxd_cpp_utils::to_wx(size),
        style
    );
    
    return reinterpret_cast<wxd_StaticText_t*>(stext);
}

WXD_EXPORTED void wxd_StaticText_Destroy(wxd_StaticText_t* stext) {
    // Assumes stext is a top-level window, which is unlikely.
    // Usually, child windows are destroyed by their parents.
    // This function might not be needed if Drop logic handles parentage.
    // For safety, we'll call Destroy() which is safe for children too.
     if (!stext) return;
     wxStaticText* wx_stext = reinterpret_cast<wxStaticText*>(stext);
     wx_stext->Destroy();
}

WXD_EXPORTED void wxd_StaticText_SetLabel(wxd_StaticText_t* stext, const char* label) {
    if (!stext) return;
    wxStaticText* wx_stext = reinterpret_cast<wxStaticText*>(stext);
    wxString wx_label = wxString::FromUTF8(label ? label : "");
    wx_stext->SetLabel(wx_label);
}

WXD_EXPORTED int wxd_StaticText_GetLabel(wxd_StaticText_t* stext, char* buffer, int buffer_len) {
    wxStaticText* wx_stext = reinterpret_cast<wxStaticText*>(stext);
    if (!wx_stext) {
         // To match previous logic of wxd_Button_GetLabel which returned 0 on error for button ptr.
         // but for consistency with copy_wxstring_to_buffer, if stext is null, we should probably still
         // indicate how much an empty string would take, or a dedicated error. Let's return 0 for null widget.
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return 0; // For null widget ptr
    }
    // The check `!buffer || buffer_len <= 0` is handled by the utility if we want to return needed length.
    // The original code returned -1. Let's align with the button's logic to return needed_len+1 or 0/error.
    // If buffer is null or buffer_len is 0, copy_wxstring_to_buffer returns needed length.
    // If stext is null, this won't be reached. The old code returned -1 on bad buffer.
    // Our utility now handles bad buffer by returning needed length.

    wxString label = wx_stext->GetLabel();
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(label, buffer, (size_t)buffer_len);
    
    // The old logic for this function returned 0 on success, and required_len if buffer was too small.
    // This is different from other GetLabel/GetValue functions.
    // Let's make it consistent: return required_len+1 if buffer is too small/null, or 0 on success if copied.
    if (buffer && (size_t)buffer_len > needed_len_no_null) {
        return 0; // Success, copied fully
    } else {
        return (int)(needed_len_no_null + 1); // Buffer too small or null, return needed size (incl. null)
    }
}

} // extern "C" 
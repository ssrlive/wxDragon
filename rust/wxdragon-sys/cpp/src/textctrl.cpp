#include "wx/wxprec.h"

#ifndef WX_PRECOMP
    #include "wx/wx.h"
#endif

#include "wx/textctrl.h"
#include "wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxTextCtrl
WXD_EXPORTED wxd_TextCtrl_t* wxd_TextCtrl_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    const char* value, 
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    wxTextCtrl* ctrl = new wxTextCtrl(
        parentWin, 
        id, 
        wxString::FromUTF8(value ? value : ""),
        wxd_cpp_utils::to_wx(pos),
        wxd_cpp_utils::to_wx(size),
        style
    );
    return (wxd_TextCtrl_t*)ctrl;
}

// Set the value of the wxTextCtrl
WXD_EXPORTED void wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

// Get the value of the wxTextCtrl
WXD_EXPORTED int wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len) {
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) {
        if (buffer && buffer_len > 0) buffer[0] = '\0';
        return 0; // Return 0 for null widget
    }
    
    wxString value = ctrl->GetValue();
    size_t needed_len_no_null = wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
    
    // Return required length + 1 (consistent with button/statictext refactoring)
    return (int)(needed_len_no_null + 1);
}

} // extern "C" 
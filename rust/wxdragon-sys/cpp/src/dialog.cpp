#include "wxdragon.h"
#include "wx/wx.h"
#include "wx/dialog.h"

extern "C" {

int wxd_Dialog_ShowModal(wxd_Dialog* self) {
    if (!self) return wxID_NONE; // Or some other error indicator, wxDialog::ShowModal returns int
    return ((wxDialog*)self)->ShowModal();
}

void wxd_Dialog_EndModal(wxd_Dialog* self, int retCode) {
    if (!self) return;
    ((wxDialog*)self)->EndModal(retCode);
}

// Note: wxDialog itself is usually not created directly with a simple 'Create' function in this C API.
// Derived dialogs (like wxMessageDialog) will have their own creation functions that return a wxd_Dialog* or wxd_SpecificDialog* castable to wxd_Dialog*.
// Destruction is handled by wxd_Window_Destroy, as wxDialog inherits from wxWindow.

} // extern "C" 
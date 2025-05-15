#include "wxdragon.h"
#include <wx/wx.h>
#include <wx/datectrl.h>
#include <wx/datetime.h>

// --- wxd_DateTime_t Helpers ---

// Helper to convert wxd_DateTime_t to wxDateTime
static wxDateTime wxd_to_wx_datetime(const wxd_DateTime_t* wxd_dt) {
    if (!wxd_dt) {
        return wxDefaultDateTime;
    }
    // wxDateTime months are 0-indexed (January=0)
    return wxDateTime(static_cast<wxDateTime::wxDateTime_t>(wxd_dt->day),
                      static_cast<wxDateTime::Month>(wxd_dt->month),
                      wxd_dt->year,
                      static_cast<wxDateTime::wxDateTime_t>(wxd_dt->hour),
                      static_cast<wxDateTime::wxDateTime_t>(wxd_dt->minute),
                      static_cast<wxDateTime::wxDateTime_t>(wxd_dt->second));
}

// Helper to convert wxDateTime to wxd_DateTime_t
static wxd_DateTime_t wx_to_wxd_datetime(const wxDateTime& wx_dt) {
    wxd_DateTime_t wxd_dt;
    if (!wx_dt.IsValid()) {
        // Match Rust's DateTime::default_value() for an invalid date
        // Typically, IsValid() would be checked by the caller after this conversion.
        // Or, set to a known invalid pattern if desired, e.g., all -1 or 0s.
        // For now, populate with wxDateTime's components even if invalid.
        wxd_dt.year = wx_dt.GetYear();
        wxd_dt.month = static_cast<short>(wx_dt.GetMonth()); // wxDateTime::Month is 0-indexed
        wxd_dt.day = static_cast<short>(wx_dt.GetDay());
        wxd_dt.hour = static_cast<short>(wx_dt.GetHour());
        wxd_dt.minute = static_cast<short>(wx_dt.GetMinute());
        wxd_dt.second = static_cast<short>(wx_dt.GetSecond());
    } else {
        wxd_dt.year = wx_dt.GetYear();
        wxd_dt.month = static_cast<short>(wx_dt.GetMonth()); // wxDateTime::Month is 0-indexed
        wxd_dt.day = static_cast<short>(wx_dt.GetDay());
        wxd_dt.hour = static_cast<short>(wx_dt.GetHour());
        wxd_dt.minute = static_cast<short>(wx_dt.GetMinute());
        wxd_dt.second = static_cast<short>(wx_dt.GetSecond());
    }
    return wxd_dt;
}

// --- wxDatePickerCtrl Functions ---

WXD_EXPORTED wxd_DatePickerCtrl_t* wxd_DatePickerCtrl_Create(wxd_Window_t* parent,
                                                           int id,
                                                           const wxd_DateTime_t* dt,
                                                           wxd_Point pos,      // Pass by value
                                                           wxd_Size size,       // Pass by value
                                                           int64_t style) {
    wxWindow* wx_parent = (wxWindow*)parent;
    wxPoint wx_pos = wxPoint(pos.x, pos.y); // Use directly
    wxSize wx_size = wxSize(size.width, size.height); // Use directly
    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime;

    // Ensure default style if none provided, or if style is 0 and it's not intended
    if (style == 0) { // Assuming 0 is not a valid combination of styles that means "no style"
        style = wxDP_DEFAULT | wxDP_SHOWCENTURY; // A sensible default
    }
    
    wxDatePickerCtrl* wx_picker = new wxDatePickerCtrl(wx_parent,
                                                       id,
                                                       wx_dt_val,
                                                       wx_pos,
                                                       wx_size,
                                                       style);
    return (wxd_DatePickerCtrl_t*)wx_picker;
}

WXD_EXPORTED wxd_DateTime_t wxd_DatePickerCtrl_GetValue(wxd_DatePickerCtrl_t* self) {
    wxDatePickerCtrl* wx_picker = (wxDatePickerCtrl*)self;
    if (!wx_picker) {
        // Return an invalid/default DateTime if self is null
        return wx_to_wxd_datetime(wxDefaultDateTime);
    }
    return wx_to_wxd_datetime(wx_picker->GetValue());
}

WXD_EXPORTED void wxd_DatePickerCtrl_SetValue(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt) {
    wxDatePickerCtrl* wx_picker = (wxDatePickerCtrl*)self;
    if (!wx_picker) return;

    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime; // wxDatePickerCtrl handles wxDefaultDateTime correctly (clears if style allows)
    wx_picker->SetValue(wx_dt_val);
}

WXD_EXPORTED bool wxd_DatePickerCtrl_GetRange(wxd_DatePickerCtrl_t* self, wxd_DateTime_t* dt1, wxd_DateTime_t* dt2) {
    wxDatePickerCtrl* wx_picker = (wxDatePickerCtrl*)self;
    if (!wx_picker) return false;

    wxDateTime wx_dt1, wx_dt2;
    bool result = wx_picker->GetRange(&wx_dt1, &wx_dt2);

    if (dt1) {
        *dt1 = wx_to_wxd_datetime(wx_dt1);
    }
    if (dt2) {
        *dt2 = wx_to_wxd_datetime(wx_dt2);
    }
    return result;
}

WXD_EXPORTED void wxd_DatePickerCtrl_SetRange(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt1, const wxd_DateTime_t* dt2) {
    wxDatePickerCtrl* wx_picker = (wxDatePickerCtrl*)self;
    if (!wx_picker) return;

    wxDateTime wx_dt1_val = dt1 ? wxd_to_wx_datetime(dt1) : wxDefaultDateTime;
    wxDateTime wx_dt2_val = dt2 ? wxd_to_wx_datetime(dt2) : wxDefaultDateTime;
    
    // Passing wxDefaultDateTime to SetRange effectively means "no bound" for that side.
    wx_picker->SetRange(wx_dt1_val, wx_dt2_val);
}

// Event type constant for DATE_CHANGED
// This should be defined in wxdragon.h in WXDEventTypeCEnum, 
// and its value extracted by const_extractor.
// For completeness, its C++ equivalent is wxEVT_DATE_CHANGED.
// The Rust side will use ffi::WXD_EVENT_TYPE_DATE_CHANGED. 
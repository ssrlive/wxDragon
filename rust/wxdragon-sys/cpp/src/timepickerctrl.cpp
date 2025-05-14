#include "wxdragon.h"
#include <wx/wx.h>
#include <wx/timectrl.h>
#include <wx/datetime.h>

// Use the same helpers from datepickerctrl.cpp

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

// --- wxTimePickerCtrl Functions ---

WXD_EXPORTED wxd_TimePickerCtrl_t* wxd_TimePickerCtrl_Create(wxd_Window_t* parent,
                                                           int id,
                                                           const wxd_DateTime_t* dt,
                                                           wxd_Point pos,      // Pass by value
                                                           wxd_Size size,       // Pass by value
                                                           long style) {
    wxWindow* wx_parent = (wxWindow*)parent;
    wxPoint wx_pos = wxPoint(pos.x, pos.y); // Use directly
    wxSize wx_size = wxSize(size.width, size.height); // Use directly
    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime;

    // Ensure default style if none provided
    if (style == 0) {
        style = wxTP_DEFAULT; // Default style for time picker
    }
    
    wxTimePickerCtrl* wx_picker = new wxTimePickerCtrl(wx_parent,
                                                      id,
                                                      wx_dt_val,
                                                      wx_pos,
                                                      wx_size,
                                                      style);
    return (wxd_TimePickerCtrl_t*)wx_picker;
}

WXD_EXPORTED wxd_DateTime_t wxd_TimePickerCtrl_GetValue(wxd_TimePickerCtrl_t* self) {
    wxTimePickerCtrl* wx_picker = (wxTimePickerCtrl*)self;
    if (!wx_picker) {
        // Return an invalid/default DateTime if self is null
        return wx_to_wxd_datetime(wxDefaultDateTime);
    }
    return wx_to_wxd_datetime(wx_picker->GetValue());
}

WXD_EXPORTED void wxd_TimePickerCtrl_SetValue(wxd_TimePickerCtrl_t* self, const wxd_DateTime_t* dt) {
    wxTimePickerCtrl* wx_picker = (wxTimePickerCtrl*)self;
    if (!wx_picker) return;

    wxDateTime wx_dt_val = dt ? wxd_to_wx_datetime(dt) : wxDefaultDateTime;
    wx_picker->SetValue(wx_dt_val);
}

// Event type constant for TIME_CHANGED
// This should be defined in wxdragon.h in WXDEventTypeCEnum,
// and its value extracted by const_extractor.
// For completeness, its C++ equivalent is wxEVT_TIME_CHANGED.
// The Rust side will use ffi::WXD_EVENT_TYPE_TIME_CHANGED. 
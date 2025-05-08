#include "wxdragon.h"
#include <wx/calctrl.h>
#include <wx/datetime.h> // For wxDateTime

// Helper to convert wxd_DateTime_t to wxDateTime
static wxDateTime wxd_to_wx_datetime(const wxd_DateTime_t* wxd_dt) {
    if (!wxd_dt) {
        // If no date is provided, wxCalendarCtrl defaults to today, so we can reflect that.
        // Or, if wxd_DateTime_t cannot represent an "unset" state distinctly from a valid date (e.g. all zeros)
        // then this needs careful handling based on how Rust side sends "no initial date".
        // For now, assuming NULL means "use wxWidgets default", which is wxDateTime::Today() for wxCalendarCtrl constructor.
        // For SetDate, a NULL might be an error or mean "no change", depending on API design.
        // Let's assume for Create, NULL means "default", for SetDate it means "error/invalid".
        return wxDateTime::Today(); 
    }
    // wxDateTime month is 0-11 (wxDateTime::Jan, wxDateTime::Feb, ...)
    // wxd_DateTime_t.month is 0-11 as per its definition in wxdragon.h (matching wxDateTime)
    return wxDateTime((unsigned short)wxd_dt->day, (wxDateTime::Month)wxd_dt->month, wxd_dt->year,
                      (unsigned short)wxd_dt->hour, (unsigned short)wxd_dt->minute, (unsigned short)wxd_dt->second);
}

// Helper to convert wxDateTime to wxd_DateTime_t
static wxd_DateTime_t wx_to_wxd_datetime(const wxDateTime& wx_dt) {
    wxd_DateTime_t wxd_dt;
    wxd_dt.day = wx_dt.GetDay(wxDateTime::Local);
    wxd_dt.month = wx_dt.GetMonth(wxDateTime::Local); // wxDateTime::Month is 0-indexed
    wxd_dt.year = wx_dt.GetYear(wxDateTime::Local);
    wxd_dt.hour = wx_dt.GetHour(wxDateTime::Local);
    wxd_dt.minute = wx_dt.GetMinute(wxDateTime::Local);
    wxd_dt.second = wx_dt.GetSecond(wxDateTime::Local);
    return wxd_dt;
}

extern "C" {

WXD_EXPORTED wxd_CalendarCtrl_t* wxd_CalendarCtrl_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    const wxd_DateTime_t* date, // Initial date, can be NULL for default (current date)
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    if (!parent) return NULL; // Parent is mandatory for a control
    wxWindow* wx_parent = (wxWindow*)parent;
    
    wxDateTime initialDate;
    if (date && date->year != 0) { // Check if date is somewhat initialized, year 0 could be a sentinel
        initialDate = wxd_to_wx_datetime(date);
    } else {
        initialDate = wxDateTime::Today(); // Default to today if date is NULL or uninitialized
    }

    wxCalendarCtrl* ctrl = new wxCalendarCtrl(
        wx_parent,
        id,
        initialDate,
        wxPoint(pos.x, pos.y),
        wxSize(size.width, size.height),
        style
        // Consider adding wxCAL_SHOW_HOLIDAYS or other defaults if desired, or leave to style
    );
    return (wxd_CalendarCtrl_t*)ctrl;
}

WXD_EXPORTED bool wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date) {
    if (!self || !date || date->year == 0) return false; // Ensure valid pointers and a somewhat valid date
    wxCalendarCtrl* ctrl = (wxCalendarCtrl*)self;
    return ctrl->SetDate(wxd_to_wx_datetime(date));
}

WXD_EXPORTED wxd_DateTime_t wxd_CalendarCtrl_GetDate(wxd_CalendarCtrl_t* self) {
    if (!self) {
        // Return an invalid or default date
        // Year 0 can signify an invalid/uninitialized wxd_DateTime_t
        wxd_DateTime_t dt_invalid = {0, 0, 0, 0, 0, 0}; 
        return dt_invalid;
    }
    wxCalendarCtrl* ctrl = (wxCalendarCtrl*)self;
    return wx_to_wxd_datetime(ctrl->GetDate());
}

// For wxCalendarEvent, which is a wxDateEvent
WXD_EXPORTED wxd_DateTime_t wxd_CalendarEvent_GetDate(wxd_Event_t* event) {
    if (!event) {
        wxd_DateTime_t dt_invalid = {0, 0, 0, 0, 0, 0};
        return dt_invalid;
    }
    // event is an opaque pointer to a wxEvent or derived class instance.
    // First, cast it to wxEvent*.
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(event);
    // Then, cast to the specific event type. Since this function is
    // wxd_CalendarEvent_GetDate, we expect a wxCalendarEvent.
    // wxCalendarEvent derives from wxDateEvent.
    wxCalendarEvent* calEvent = static_cast<wxCalendarEvent*>(baseEvent); 
    // For safety, one might use dynamic_cast and check for null if the type isn't guaranteed by call context.
    // However, given the C function's name, static_cast is acceptable if Rust ensures correct usage.
    return wx_to_wxd_datetime(calEvent->GetDate());
}

} // extern "C" 
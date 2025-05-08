#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/datetime.h>

extern "C" {

// Convert Rust struct to wxDateTime and return pointer
wxd_DateTime_t wxd_DateTime_FromComponents(
    int year,
    unsigned short month, // wxDateTime month is 0-based
    short day,
    short hour,
    short minute,
    short second
) {
    wxDateTime dt;
    // Note: wxDateTime::Set takes day, month, year
    dt.Set(static_cast<wxDateTime::wxDateTime_t>(day), 
           static_cast<wxDateTime::Month>(month), 
           year,
           static_cast<wxDateTime::wxDateTime_t>(hour),
           static_cast<wxDateTime::wxDateTime_t>(minute),
           static_cast<wxDateTime::wxDateTime_t>(second));
    
    wxd_DateTime_t result;
    if (dt.IsValid()) {
        result.day = dt.GetDay();
        result.month = dt.GetMonth(); // wxDateTime::Month enum (0-based)
        result.year = dt.GetYear();
        result.hour = dt.GetHour();
        result.minute = dt.GetMinute();
        result.second = dt.GetSecond();
    } else {
        // Return an invalid representation
        result.day = 0;
        result.month = 0;
        result.year = 0;
        result.hour = 0;
        result.minute = 0;
        result.second = 0;
    }
    return result;
}

wxd_DateTime_t wxd_DateTime_Now() {
    wxDateTime now = wxDateTime::Now();
    wxd_DateTime_t result;
    result.day = now.GetDay();
    result.month = now.GetMonth();
    result.year = now.GetYear();
    result.hour = now.GetHour();
    result.minute = now.GetMinute();
    result.second = now.GetSecond();
    return result;
}

// Returns an invalid wxDateTime representation
wxd_DateTime_t wxd_DateTime_Default() {
    wxd_DateTime_t result;
    result.day = 0;
    result.month = 0; 
    result.year = 0;
    result.hour = 0;
    result.minute = 0;
    result.second = 0;
    return result;
}

bool wxd_DateTime_IsValid(const wxd_DateTime_t* dt) {
    if (!dt) return false;
    // Reconstruct a wxDateTime to check validity
    wxDateTime::Month wx_month = static_cast<wxDateTime::Month>(dt->month);
    // Basic check: year 0 is often invalid, months must be 0-11
    if (dt->year == 0 || dt->month > 11) return false; 
    
    wxDateTime temp_dt(
        static_cast<wxDateTime::wxDateTime_t>(dt->day), 
        wx_month, 
        dt->year,
        static_cast<wxDateTime::wxDateTime_t>(dt->hour),
        static_cast<wxDateTime::wxDateTime_t>(dt->minute),
        static_cast<wxDateTime::wxDateTime_t>(dt->second)
    );
    return temp_dt.IsValid();
}

} // extern "C"
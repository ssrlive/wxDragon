#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/datetime.h>

extern "C" {

// Convert Rust struct to wxDateTime and return pointer
wxd_DateTime_t wxd_DateTime_FromComponents(
    int year,
    unsigned short month, // Already 0-based, adjusted in Rust wrapper
    short day,
    short hour,
    short minute,
    short second
) {
    // Initialize an invalid result
    wxd_DateTime_t invalid = {0};
    
    // Validate parameters according to wxDateTime::Set requirements
    if (year <= 0 || month >= 12 || day <= 0 || day > 31 ||
        hour < 0 || hour >= 24 || minute < 0 || minute >= 60 || second < 0 || second >= 60) {
        return invalid;
    }
    
    wxDateTime dt;
    
    // Try to create a wxDateTime (avoid using Set() directly)
    try {
        // Try a different approach - explicitly create wxDateTime
        dt = wxDateTime((wxDateTime::wxDateTime_t)day, 
                        (wxDateTime::Month)month, 
                        year,
                        (wxDateTime::wxDateTime_t)hour,
                        (wxDateTime::wxDateTime_t)minute,
                        (wxDateTime::wxDateTime_t)second);
    }
    catch (const std::exception&) {
        return invalid;
    }
    catch (...) {
        return invalid;
    }
    
    // Return the result
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
        result = invalid;
    }
    
    return result;
}

wxd_DateTime_t wxd_DateTime_Now() {
    wxDateTime now = wxDateTime::Now();
    wxd_DateTime_t result;
    
    if (now.IsValid()) {
        result.day = now.GetDay();
        result.month = now.GetMonth();
        result.year = now.GetYear();
        result.hour = now.GetHour();
        result.minute = now.GetMinute();
        result.second = now.GetSecond();
    } else {
        // This should never happen with Now()
        result.day = 0;
        result.month = 0; 
        result.year = 0;
        result.hour = 0;
        result.minute = 0;
        result.second = 0;
    }
    
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
    if (!dt) {
        return false;
    }
    
    // Basic check: year 0 is often invalid, months must be 0-11
    if (dt->year == 0 || dt->month > 11) {
        return false; 
    }
    
    // Try to create a wxDateTime to check validity
    try {
        // Don't use the wxDateTime constructor directly, as it might be what's failing
        // Instead, let's use individual checks
        if (dt->month > 11 || dt->day < 1 || dt->day > 31 || 
            dt->hour >= 24 || dt->minute >= 60 || dt->second >= 60) {
            return false;
        }
        
        // For February, check leap year rules
        if (dt->month == 1) {  // February (0-based)
            bool isLeap = (dt->year % 4 == 0) && 
                          ((dt->year % 100 != 0) || (dt->year % 400 == 0));
            int maxDays = isLeap ? 29 : 28;
            
            if (dt->day > maxDays) {
                return false;
            }
        }
        
        // For months with 30 days
        if ((dt->month == 3 || dt->month == 5 || dt->month == 8 || dt->month == 10) && 
            dt->day > 30) {
            return false;
        }
        
        return true;
    }
    catch (const std::exception&) {
        return false;
    }
    catch (...) {
        return false;
    }
}

} // extern "C"
#ifndef WXD_CALENDARCTRL_H
#define WXD_CALENDARCTRL_H

#include "../wxd_types.h"

// --- CalendarCtrl Functions ---
WXD_EXPORTED wxd_CalendarCtrl_t* wxd_CalendarCtrl_Create(wxd_Window_t* parent, wxd_Id id, const wxd_DateTime_t* date, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_CalendarCtrl_SetDate(wxd_CalendarCtrl_t* self, const wxd_DateTime_t* date);
WXD_EXPORTED wxd_DateTime_t wxd_CalendarCtrl_GetDate(wxd_CalendarCtrl_t* self);

#endif // WXD_CALENDARCTRL_H 
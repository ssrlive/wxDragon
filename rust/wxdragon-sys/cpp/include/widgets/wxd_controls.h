#ifndef WXD_CONTROLS_H
#define WXD_CONTROLS_H

#include "../wxd_types.h"

// --- Button ---
WXD_EXPORTED wxd_Button_t* wxd_Button_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_Button_Destroy(wxd_Button_t* button); // Note: Consider if needed, generic wxd_Window_Destroy might suffice
WXD_EXPORTED void wxd_Button_SetLabel(wxd_Button_t* button, const char* label);
WXD_EXPORTED int wxd_Button_GetLabel(wxd_Button_t* button, char* buffer, int buffer_len);

// --- StaticText ---
WXD_EXPORTED wxd_StaticText_t* wxd_StaticText_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_StaticText_Destroy(wxd_StaticText_t* stext); // Generic might suffice
WXD_EXPORTED void wxd_StaticText_SetLabel(wxd_StaticText_t* stext, const char* label);
WXD_EXPORTED int wxd_StaticText_GetLabel(wxd_StaticText_t* stext, char* buffer, int buffer_len);

// --- TextCtrl ---
WXD_EXPORTED wxd_TextCtrl_t* wxd_TextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value);
WXD_EXPORTED int wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len);
WXD_EXPORTED void wxd_TextCtrl_AppendText(wxd_TextCtrl_t* textCtrl, const char* text);
WXD_EXPORTED void wxd_TextCtrl_Clear(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsModified(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void wxd_TextCtrl_SetModified(wxd_TextCtrl_t* textCtrl, bool modified);
WXD_EXPORTED void wxd_TextCtrl_SetEditable(wxd_TextCtrl_t* textCtrl, bool editable);
WXD_EXPORTED bool wxd_TextCtrl_IsEditable(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetInsertionPoint(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED void wxd_TextCtrl_SetInsertionPoint(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos);
WXD_EXPORTED void wxd_TextCtrl_SetMaxLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t len);
WXD_EXPORTED wxd_Long_t wxd_TextCtrl_GetLastPosition(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsMultiLine(wxd_TextCtrl_t* textCtrl);
WXD_EXPORTED bool wxd_TextCtrl_IsSingleLine(wxd_TextCtrl_t* textCtrl);

// --- CheckBox ---
WXD_EXPORTED wxd_CheckBox_t* wxd_CheckBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_CheckBox_IsChecked(wxd_CheckBox_t* checkBox);
WXD_EXPORTED void wxd_CheckBox_SetValue(wxd_CheckBox_t* checkBox, bool value);

// --- RadioButton ---
WXD_EXPORTED wxd_RadioButton_t* wxd_RadioButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_RadioButton_GetValue(wxd_RadioButton_t* radio);
WXD_EXPORTED void wxd_RadioButton_SetValue(wxd_RadioButton_t* radio, bool value);

// --- ToggleButton ---
WXD_EXPORTED wxd_ToggleButton_t* wxd_ToggleButton_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED bool wxd_ToggleButton_GetValue(wxd_ToggleButton_t* tglbtn);
WXD_EXPORTED void wxd_ToggleButton_SetValue(wxd_ToggleButton_t* tglbtn, bool state);
WXD_EXPORTED void wxd_ToggleButton_SetLabel(wxd_ToggleButton_t* tglbtn, const char* label);
WXD_EXPORTED int wxd_ToggleButton_GetLabel(wxd_ToggleButton_t* tglbtn, char* buffer, int buffer_len);

// --- Gauge ---
WXD_EXPORTED wxd_Gauge_t *wxd_Gauge_Create(wxd_Window_t *parent, wxd_Id id, int range, int x, int y, int w, int h, wxd_Style_t style);
WXD_EXPORTED void wxd_Gauge_SetRange(wxd_Gauge_t *self, int range);
WXD_EXPORTED void wxd_Gauge_SetValue(wxd_Gauge_t *self, int value);
WXD_EXPORTED int wxd_Gauge_GetValue(const wxd_Gauge_t *self);

// --- Slider ---
WXD_EXPORTED wxd_Slider_t* wxd_Slider_Create(wxd_Window_t* parent, wxd_Id id, int value, int minValue, int maxValue, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED int wxd_Slider_GetValue(wxd_Slider_t* self);
WXD_EXPORTED void wxd_Slider_SetValue(wxd_Slider_t* self, int value);
WXD_EXPORTED void wxd_Slider_SetRange(wxd_Slider_t* self, int minValue, int maxValue);
WXD_EXPORTED int wxd_Slider_GetMin(wxd_Slider_t* self);
WXD_EXPORTED int wxd_Slider_GetMax(wxd_Slider_t* self);

// --- SpinCtrl ---
WXD_EXPORTED wxd_SpinCtrl_t* wxd_SpinCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos, wxd_Size size, wxd_Style_t style, int min_val, int max_val, int initial_val);
WXD_EXPORTED int wxd_SpinCtrl_GetValue(wxd_SpinCtrl_t* self);
WXD_EXPORTED void wxd_SpinCtrl_SetValue(wxd_SpinCtrl_t* self, int value);
WXD_EXPORTED void wxd_SpinCtrl_SetRange(wxd_SpinCtrl_t* self, int minVal, int maxVal);
WXD_EXPORTED int wxd_SpinCtrl_GetMin(wxd_SpinCtrl_t* self);
WXD_EXPORTED int wxd_SpinCtrl_GetMax(wxd_SpinCtrl_t* self);

// --- SpinButton ---
WXD_EXPORTED wxd_SpinButton_t* wxd_SpinButton_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED int wxd_SpinButton_GetValue(wxd_SpinButton_t* self);
WXD_EXPORTED void wxd_SpinButton_SetValue(wxd_SpinButton_t* self, int value);
WXD_EXPORTED void wxd_SpinButton_SetRange(wxd_SpinButton_t* self, int minVal, int maxVal);
WXD_EXPORTED int wxd_SpinButton_GetMin(wxd_SpinButton_t* self);
WXD_EXPORTED int wxd_SpinButton_GetMax(wxd_SpinButton_t* self);

// --- SearchCtrl ---
WXD_EXPORTED wxd_SearchCtrl_t *wxd_SearchCtrl_Create(wxd_Window_t *parent, int id, const char *value, int x, int y, int w, int h, long style);
WXD_EXPORTED void wxd_SearchCtrl_ShowSearchButton(wxd_SearchCtrl_t *self, bool show);
WXD_EXPORTED bool wxd_SearchCtrl_IsSearchButtonVisible(wxd_SearchCtrl_t *self);
WXD_EXPORTED void wxd_SearchCtrl_ShowCancelButton(wxd_SearchCtrl_t *self, bool show);
WXD_EXPORTED bool wxd_SearchCtrl_IsCancelButtonVisible(wxd_SearchCtrl_t *self);
WXD_EXPORTED wxd_Control_t* wxd_SearchCtrl_GetCancelButton(wxd_SearchCtrl_t* self);
WXD_EXPORTED void wxd_SearchCtrl_SetMenu(wxd_SearchCtrl_t* self, wxd_Menu_t* menu);
WXD_EXPORTED wxd_Menu_t* wxd_SearchCtrl_GetMenu(wxd_SearchCtrl_t* self);

// --- HyperlinkCtrl ---
WXD_EXPORTED wxd_HyperlinkCtrl_t *wxd_HyperlinkCtrl_Create(wxd_Window_t *parent, int id, const char *label, const char *url, int x, int y, int w, int h, long style);
WXD_EXPORTED const char *wxd_HyperlinkCtrl_GetURL(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetURL(wxd_HyperlinkCtrl_t *self, const char *url);
WXD_EXPORTED bool wxd_HyperlinkCtrl_GetVisited(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetVisited(wxd_HyperlinkCtrl_t *self, bool visited);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetHoverColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetHoverColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetNormalColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetNormalColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);
WXD_EXPORTED unsigned long wxd_HyperlinkCtrl_GetVisitedColour(wxd_HyperlinkCtrl_t *self);
WXD_EXPORTED void wxd_HyperlinkCtrl_SetVisitedColour(wxd_HyperlinkCtrl_t *self, unsigned long colour);

// --- ActivityIndicator ---
WXD_EXPORTED wxd_ActivityIndicator_t *wxd_ActivityIndicator_Create(wxd_Window_t *parent, int id, int x, int y, int w, int h, long style);
WXD_EXPORTED void wxd_ActivityIndicator_Start(wxd_ActivityIndicator_t *self);
WXD_EXPORTED void wxd_ActivityIndicator_Stop(wxd_ActivityIndicator_t *self);
WXD_EXPORTED bool wxd_ActivityIndicator_IsRunning(wxd_ActivityIndicator_t *self);

// --- AnimationCtrl ---
WXD_EXPORTED wxd_AnimationCtrl_t* wxd_AnimationCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* animation_file, int x, int y, int width, int height, long style, const char* name);
WXD_EXPORTED bool wxd_AnimationCtrl_Play(wxd_AnimationCtrl_t* self);
WXD_EXPORTED void wxd_AnimationCtrl_Stop(wxd_AnimationCtrl_t* self);
WXD_EXPORTED bool wxd_AnimationCtrl_IsPlaying(wxd_AnimationCtrl_t* self);
WXD_EXPORTED bool wxd_AnimationCtrl_LoadFile(wxd_AnimationCtrl_t* self, const char* animation_file);
WXD_EXPORTED bool wxd_AnimationCtrl_LoadFromBytes(wxd_AnimationCtrl_t* self, const unsigned char* data, size_t len);

// --- CommandLinkButton ---
WXD_EXPORTED wxd_CommandLinkButton_t* wxd_CommandLinkButton_Create(wxd_Window_t* parent, wxd_Id id, const char* mainLabel, const char* note, wxd_Point pos, wxd_Size size, wxd_Style_t style);
WXD_EXPORTED void wxd_CommandLinkButton_SetNote(wxd_CommandLinkButton_t* self, const char* note);

// --- StaticBitmap ---
WXD_EXPORTED wxd_StaticBitmap_t* wxd_StaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id, wxd_Bitmap_t* bitmap, wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name);
WXD_EXPORTED void wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap_t* self, wxd_Bitmap_t* bitmap);

// --- StaticLine ---
WXD_EXPORTED wxd_StaticLine_t* wxd_StaticLine_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name);

// --- ScrollBar ---
WXD_EXPORTED wxd_ScrollBar_t* wxd_ScrollBar_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name);
WXD_EXPORTED void wxd_ScrollBar_SetScrollbar(wxd_ScrollBar_t* self, int position, int thumbSize, int range, int pageSize, bool refresh);
WXD_EXPORTED int wxd_ScrollBar_GetThumbPosition(wxd_ScrollBar_t* self);

// --- SpinCtrlDouble (Often grouped with SpinCtrl) ---
WXD_EXPORTED wxd_SpinCtrlDouble_t *wxd_SpinCtrlDouble_Create(wxd_Window_t *parent, int id, const char *value, int x, int y, int w, int h, long style, double min_val, double max_val, double initial_val, double inc);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetValue(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetValue(wxd_SpinCtrlDouble_t *self, double value);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetRange(wxd_SpinCtrlDouble_t *self, double min_val, double max_val);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetMin(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetMax(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetIncrements(wxd_SpinCtrlDouble_t *self, double inc);
WXD_EXPORTED double wxd_SpinCtrlDouble_GetIncrement(wxd_SpinCtrlDouble_t *self);
WXD_EXPORTED void wxd_SpinCtrlDouble_SetDigits(wxd_SpinCtrlDouble_t *self, unsigned int digits);
WXD_EXPORTED unsigned int wxd_SpinCtrlDouble_GetDigits(wxd_SpinCtrlDouble_t *self);

// --- BitmapButton ---
WXD_EXPORTED wxd_BitmapButton_t* wxd_BitmapButton_Create(
    wxd_Window_t* parent, 
    wxd_Id id, 
    wxd_Bitmap_t* bitmap,         // Main bitmap (normal state)
    wxd_Point pos, 
    wxd_Size size, 
    wxd_Style_t style, 
    const char* name,
    wxd_Bitmap_t* bitmap_disabled, // Disabled state bitmap (can be NULL)
    wxd_Bitmap_t* bitmap_focus,    // Focus state bitmap (can be NULL)
    wxd_Bitmap_t* bitmap_hover     // Hover state bitmap (can be NULL)
);

#endif // WXD_CONTROLS_H 
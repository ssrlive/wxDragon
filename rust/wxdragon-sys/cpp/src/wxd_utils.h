#ifndef WXD_UTILS_H
#define WXD_UTILS_H

#include <wx/gdicmn.h> // For wxPoint, wxSize, wxDefaultPosition, wxDefaultSize
#include <wx/string.h> // For wxString
#include "../include/wxdragon.h" // For wxd_Point, wxd_Size
#include <wx/colour.h> // For wxColour type

namespace wxd_cpp_utils {

// Inline helper function to convert wxd_Point to wxPoint
inline wxPoint to_wx(const wxd_Point& p) {
    if (p.x == -1 && p.y == -1) { // Common convention for default pos
        return wxDefaultPosition;
    }
    return wxPoint(p.x, p.y);
}

// Inline helper function to convert wxd_Size to wxSize
inline wxSize to_wx(const wxd_Size& s) {
    if (s.width == -1 && s.height == -1) { // Common convention for default size
        return wxDefaultSize;
    }
    return wxSize(s.width, s.height);
}

/**
 * @brief Copies a wxString to a C char buffer, ensuring null termination.
 *
 * Converts the wxString to UTF-8 before copying.
 * If the buffer is too small, the string will be truncated, but still null-terminated.
 *
 * @param str The source wxString.
 * @param buffer The destination char buffer.
 * @param buffer_len The total size of the destination buffer (including space for null terminator).
 * @return The number of bytes that would be written if the buffer was large enough 
 *         (excluding the null terminator), similar to snprintf. This is the length of str.ToUTF8().
 */
size_t copy_wxstring_to_buffer(const wxString& str, char* buffer, size_t buffer_len);

}

// Helper to convert wxd_Colour_t representation (unsigned long RGBA) to wxColour
wxColour wxdColourToWxColour(unsigned long wxd_colour_val);

// Helper to convert wxColour to wxd_Colour_t representation (unsigned long RGBA)
unsigned long wxColourToWxdColour(const wxColour& wx_colour_obj);

#endif // WXD_UTILS_H 
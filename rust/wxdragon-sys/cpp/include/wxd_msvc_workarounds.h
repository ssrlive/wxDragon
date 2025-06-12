#ifndef WXD_MSVC_WORKAROUNDS_H
#define WXD_MSVC_WORKAROUNDS_H

// This file contains workarounds for MSVC compilation issues with wxWidgets
// It should be included before any wxWidgets headers

#ifdef _MSC_VER

// Include standard headers first
#include <stdint.h>
#include <stddef.h>

// Force define wxIntPtr types before wxWidgets headers are processed
#ifndef wxIntPtr
    #ifdef _WIN64
        typedef intptr_t wxIntPtr;
        typedef uintptr_t wxUIntPtr;
    #else
        typedef int wxIntPtr;
        typedef unsigned int wxUIntPtr;
    #endif
    #define WXINTPTR_DEFINED 1
#endif

// Force Windows platform detection
#ifndef __WXMSW__
    #define __WXMSW__ 1
#endif

#ifndef _WIN32
    #define _WIN32 1
#endif

#ifndef WIN32
    #define WIN32 1
#endif

#ifndef _WINDOWS
    #define _WINDOWS 1
#endif

// Ensure Unicode support
#ifndef wxUSE_UNICODE
    #define wxUSE_UNICODE 1
#endif

// Fix for missing type specifiers
#ifndef HAVE_INTPTR_T
    #define HAVE_INTPTR_T 1
#endif

#endif // _MSC_VER

#endif // WXD_MSVC_WORKAROUNDS_H 
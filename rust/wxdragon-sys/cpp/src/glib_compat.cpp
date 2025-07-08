#include <glib.h>

// Compatibility function for g_string_free_and_steal
// This function is available in GLib 2.76+, but the prebuilt wxWidgets 
// libraries expect it to be available. We provide a fallback implementation.

extern "C" {

// Check if g_string_free_and_steal is already available
// We'll only define it if it's not available in the current GLib version
#if !GLIB_CHECK_VERSION(2, 76, 0)
char* g_string_free_and_steal(GString* string) {
    if (string == nullptr) {
        return nullptr;
    }
    
    // Extract the character data before freeing the GString structure
    char* data = string->str;
    
    // Free the GString structure but keep the character data
    // This mimics the behavior of g_string_free with free_segment=FALSE
    g_string_free(string, FALSE);
    
    return data;
}
#endif

} // extern "C" 
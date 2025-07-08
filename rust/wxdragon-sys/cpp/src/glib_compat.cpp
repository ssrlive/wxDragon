// Self-contained GLib compatibility for g_string_free_and_steal
// This avoids needing glib.h headers during compilation
// We define the minimal structures and function prototypes needed

extern "C" {

// Minimal GString structure definition (matches GLib's internal structure)
typedef struct {
    char* str;
    unsigned long len;
    unsigned long allocated_len;
} GString;

// Declare g_string_free as an external function (will be provided by system GLib)
extern char* g_string_free(GString* string, int free_segment);

// Provide g_string_free_and_steal implementation
// This function is available in GLib 2.76+, but older systems may not have it
char* g_string_free_and_steal(GString* string) {
    if (string == nullptr) {
        return nullptr;
    }
    
    // This is equivalent to g_string_free(string, FALSE)
    // which frees the GString structure but returns the character data
    return g_string_free(string, 0);
}

} // extern "C" 
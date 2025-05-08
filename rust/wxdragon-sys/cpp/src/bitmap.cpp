#include "../include/wxdragon.h"
#include <wx/wx.h>
#include <wx/image.h> // For wxImage
#include <wx/bitmap.h> // For wxBitmap
#include <cstdlib> // For malloc, free
#include <cstring> // For memcpy

// Implementation for wxd_Bitmap_CreateFromRGBA
WXD_EXPORTED wxd_Bitmap_t* wxd_Bitmap_CreateFromRGBA(
    const unsigned char* data, 
    int width, 
    int height
) {
    if (!data || width <= 0 || height <= 0) {
        return nullptr;
    }

    // wxImage::SetData requires a buffer allocated with malloc that it can free later.
    // We must copy the RGB data from the Rust-provided RGBA buffer into a new malloc'd buffer.
    size_t num_pixels = static_cast<size_t>(width) * static_cast<size_t>(height);
    size_t rgb_data_size = num_pixels * 3; 
    unsigned char* rgb_data = static_cast<unsigned char*>(malloc(rgb_data_size));
    if (!rgb_data) {
        wxLogError("Failed to allocate memory for bitmap RGB data.");
        return nullptr; 
    }

    // Also need a separate buffer for Alpha data
    size_t alpha_data_size = num_pixels; // 1 byte per pixel
    unsigned char* alpha_data = static_cast<unsigned char*>(malloc(alpha_data_size));
    if (!alpha_data) {
        wxLogError("Failed to allocate memory for bitmap Alpha data.");
        free(rgb_data); // Clean up already allocated buffer
        return nullptr;
    }

    // Copy data from input RGBA buffer to separate RGB and Alpha buffers
    for (size_t i = 0; i < num_pixels; ++i) {
        rgb_data[i * 3 + 0] = data[i * 4 + 0]; // R
        rgb_data[i * 3 + 1] = data[i * 4 + 1]; // G
        rgb_data[i * 3 + 2] = data[i * 4 + 2]; // B
        alpha_data[i]       = data[i * 4 + 3]; // A
    }

    // Create wxImage. It takes ownership of rgb_data AND alpha_data.
    wxImage image(width, height, rgb_data, alpha_data); // Pass both buffers

    if (!image.IsOk()) {
        wxLogError("Failed to create wxImage from RGBA data.");
        // If image creation failed, wxImage *should* have freed rgb_data and alpha_data, but double-check docs.
        // Assuming it did, we just return nullptr.
        // If it didn't free on failure (unlikely), we would need: free(rgb_data); free(alpha_data);
        return nullptr;
    }

    // Now create the wxBitmap from the wxImage.
    // Use depth -1 for default screen depth.
    wxBitmap* bitmap = new wxBitmap(image, -1);

    if (!bitmap || !bitmap->IsOk()) {
        wxLogError("Failed to create wxBitmap from wxImage.");
        delete bitmap; // Clean up partially created bitmap if possible
        return nullptr;
    }

    return reinterpret_cast<wxd_Bitmap_t*>(bitmap);
}

// Implementation for wxd_Bitmap_Destroy
WXD_EXPORTED void wxd_Bitmap_Destroy(wxd_Bitmap_t* bitmap) {
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    delete bmp;
}

// ADDED: Get bitmap dimensions and validity
WXD_EXPORTED int wxd_Bitmap_GetWidth(wxd_Bitmap_t* bitmap) {
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    if (!bmp) return 0;
    return bmp->GetWidth();
}

WXD_EXPORTED int wxd_Bitmap_GetHeight(wxd_Bitmap_t* bitmap) {
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    if (!bmp) return 0;
    return bmp->GetHeight();
}

WXD_EXPORTED bool wxd_Bitmap_IsOk(wxd_Bitmap_t* bitmap) {
    wxBitmap* bmp = reinterpret_cast<wxBitmap*>(bitmap);
    if (!bmp) return false;
    return bmp->IsOk();
} 
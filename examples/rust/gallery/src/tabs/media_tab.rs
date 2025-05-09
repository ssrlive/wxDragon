use image::GenericImageView;
use wxdragon::prelude::*;
use wxdragon::Bitmap;

const ANIMATION_BYTES: &[u8] = include_bytes!("../../../../../asset/dancing-ferris.gif"); // CORRECTED Path relative to this media_tab.rs file

pub struct MediaControls {
    pub panel: Panel,
    pub animation_ctrl: AnimationCtrl,
    // Add other media controls here later
}

impl MediaControls {
    pub fn bind_events(&self) {
        // Bind events for media controls if any
        self.animation_ctrl.bind(EventType::LEFT_DOWN, |_event| {
            println!("AnimationCtrl clicked - this event might not be standard for it, just for testing.");
        });
    }
}

pub fn create_media_tab(notebook: &Notebook) -> MediaControls {
    let panel = Panel::builder(notebook).build();
    let sizer = BoxSizer::builder(VERTICAL).build();

    // Determine animation size
    let animation_size = match image::load_from_memory(ANIMATION_BYTES) {
        Ok(anim_image) => {
            let (w, h) = anim_image.dimensions();
            println!("Loaded animation dimensions: {}x{}", w, h);
            Size::new(w as i32, h as i32)
        }
        Err(e) => {
            println!(
                "Failed to load animation metadata to get size: {}. Falling back to default.",
                e
            );
            Size::new(100, 100) // Fallback size
        }
    };

    let animation_ctrl = AnimationCtrl::builder(&panel)
        .with_animation_file("") // Pass empty string for file as we load from bytes
        .with_size(animation_size) // Use determined or fallback size
        .build();

    if animation_ctrl.load_from_bytes(ANIMATION_BYTES) {
        println!("Animation loaded from bytes successfully.");
        if animation_ctrl.play() {
            println!("Animation started successfully from bytes.");
        } else {
            println!("Failed to start animation even after loading from bytes.");
        }
    } else {
        println!("Failed to load animation from bytes.");
    }

    sizer.add(&animation_ctrl, 0, ALIGN_CENTER_HORIZONTAL | ALL, 10);

    let info_text = StaticText::builder(&panel)
        .with_label("Animation loaded from embedded bytes. Dancing Ferris should appear above.")
        .build();
    sizer.add(&info_text, 0, ALL, 10);

    // --- StaticBitmap Demo ---
    let hbox_bitmap_example = BoxSizer::builder(HORIZONTAL).build();
    let static_bitmap_image_bytes = include_bytes!("../../asset/simple.png"); // Path relative to media_tab.rs
    match image::load_from_memory_with_format(static_bitmap_image_bytes, image::ImageFormat::Png) {
        Ok(img) => {
            let rgba_data = img.to_rgba8();
            let (width, height) = img.dimensions();
            if let Some(bitmap_obj) = Bitmap::from_rgba(rgba_data.as_raw(), width, height) {
                let static_bitmap_ctrl = StaticBitmap::builder(&panel)
                    .with_bitmap(&bitmap_obj)
                    .with_size(Size::new(width as i32, height as i32))
                    .build();

                let bmp_label = StaticText::builder(&panel)
                    .with_label("StaticBitmap (simple.png from bytes):")
                    .build();
                hbox_bitmap_example.add(&bmp_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                hbox_bitmap_example.add(&static_bitmap_ctrl, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
            } else {
                println!("[MediaTab] Failed to create Bitmap object for StaticBitmap.");
                let bmp_error_label = StaticText::builder(&panel)
                    .with_label("StaticBitmap: Error creating Bitmap obj")
                    .build();
                hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
            }
        }
        Err(e) => {
            println!("[MediaTab] Failed to decode PNG for StaticBitmap: {}", e);
            let bmp_error_label = StaticText::builder(&panel)
                .with_label("StaticBitmap: Error decoding PNG")
                .build();
            hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
        }
    }
    sizer.add_sizer(&hbox_bitmap_example, 0, ALIGN_LEFT | ALL, 5);
    // --- End StaticBitmap Demo ---

    panel.set_sizer(sizer, true);

    MediaControls {
        panel,
        animation_ctrl,
    }
}

use wxdragon::prelude::*;
use image::GenericImageView;
use wxdragon::Bitmap;

pub struct BookControlsTab {
    pub tab_panel: Panel,
    pub treebook: Treebook,
}

pub fn create_book_controls_tab(notebook: &Notebook) -> BookControlsTab {
    let tab_panel = Panel::builder(notebook).with_style(TAB_TRAVERSAL).build();

    let treebook = Treebook::builder(&tab_panel)
        .with_id(ID_HIGHEST + 20) // Example ID
        .build();

    // Page 1: Info Page
    let info_page_panel = Panel::builder(&treebook).build();
    let info_label = StaticText::builder(&info_page_panel)
        .with_label("This is the Treebook's information page.")
        .build();
    let info_button = Button::builder(&info_page_panel)
        .with_label("Info Button")
        .build();
    let info_page_sizer = BoxSizer::builder(VERTICAL).build();
    info_page_sizer.add(&info_label, 0, ALL | EXPAND, 10);
    info_page_sizer.add(&info_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    info_page_panel.set_sizer(info_page_sizer, true);
    info_page_panel.fit();
    treebook.add_page(&info_page_panel, "Info", true, -1);

    // Page 2: Settings Page
    let settings_page_panel = Panel::builder(&treebook).build();
    let settings_label = StaticText::builder(&settings_page_panel)
        .with_label("Treebook settings would go here.")
        .build();
    let settings_button = Button::builder(&settings_page_panel)
        .with_label("Settings Button")
        .build();
    let settings_page_sizer = BoxSizer::builder(VERTICAL).build();
    settings_page_sizer.add(&settings_label, 0, ALL | EXPAND, 10);
    settings_page_sizer.add(&settings_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    settings_page_panel.set_sizer(settings_page_sizer, true);
    settings_page_panel.fit();
    let _settings_page_index = treebook.add_page(&settings_page_panel, "Settings", false, -1);

    // Sub-Page for Settings Page
    let advanced_settings_panel = Panel::builder(&treebook).build();
    let advanced_label = StaticText::builder(&advanced_settings_panel)
        .with_label("Advanced Treebook settings.")
        .build();
    let advanced_button = Button::builder(&advanced_settings_panel)
        .with_label("Advanced Button")
        .build();
    let advanced_sizer = BoxSizer::builder(VERTICAL).build();
    advanced_sizer.add(&advanced_label, 0, ALL | EXPAND, 10);
    advanced_sizer.add(&advanced_button, 0, ALL | ALIGN_CENTER_HORIZONTAL, 5);
    advanced_settings_panel.set_sizer(advanced_sizer, true);
    advanced_settings_panel.fit();
    treebook.add_sub_page(&advanced_settings_panel, "Advanced", false, -1);

    // Sizer for the main tab panel, to make the Treebook expand
    let main_tab_sizer = BoxSizer::builder(VERTICAL).build();
    main_tab_sizer.add(&treebook, 1, EXPAND | ALL, 5);

    // Example of a StaticBitmap using include_bytes!
    let hbox_bitmap_example = BoxSizer::builder(HORIZONTAL).build();
    let image_bytes = include_bytes!("../../asset/simple.png"); // Updated path relative to this file
    match image::load_from_memory_with_format(image_bytes, image::ImageFormat::Png) {
        Ok(img) => {
            let rgba_data = img.to_rgba8();
            let (width, height) = img.dimensions();
            if let Some(bitmap_obj) = Bitmap::from_rgba(rgba_data.as_raw(), width, height) {
                if let Some(static_bitmap) = StaticBitmap::builder(&tab_panel)
                    .with_bitmap(bitmap_obj) // Use with_bitmap
                    .with_size(Size::new(width as i32, height as i32))
                    .build()
                {
                    let bmp_label = StaticText::builder(&tab_panel)
                        .with_label("StaticBitmap (from bytes):")
                        .build();
                    hbox_bitmap_example.add(&bmp_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                    hbox_bitmap_example.add(&static_bitmap, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                } else {
                    println!("Failed to create StaticBitmap from Bitmap object.");
                    let bmp_error_label = StaticText::builder(&tab_panel)
                        .with_label("StaticBitmap: Error creating from Bitmap obj")
                        .build();
                    hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
                }
            } else {
                println!("Failed to create wxdragon::Bitmap from RGBA data.");
                let bmp_error_label = StaticText::builder(&tab_panel)
                    .with_label("StaticBitmap: Error creating wxBitmap from RGBA")
                    .build();
                hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
            }
        }
        Err(e) => {
            println!("Failed to decode PNG from memory: {}", e);
            let bmp_error_label = StaticText::builder(&tab_panel)
                .with_label("StaticBitmap: Error decoding PNG")
                .build();
            hbox_bitmap_example.add(&bmp_error_label, 0, ALIGN_CENTER_VERTICAL | ALL, 5);
        }
    }
    main_tab_sizer.add_sizer(&hbox_bitmap_example, 0, ALIGN_LEFT | ALL, 5);

    tab_panel.set_sizer(main_tab_sizer, true);
    tab_panel.fit();

    BookControlsTab {
        tab_panel,
        treebook,
    }
}

impl BookControlsTab {
    pub fn bind_events(&self) {
        // Bind Treebook Page Changed Event
        self.treebook.bind(EventType::TREEBOOK_PAGE_CHANGED, |event: Event| {
            println!(
                "TREEBOOK_PAGE_CHANGED Event: OldSel={}, NewSel={}, Event={:?}",
                event.get_old_selection().unwrap_or(-2),
                event.get_selection().unwrap_or(-2),
                event
            );
        });
    }
} 
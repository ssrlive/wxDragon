use wxdragon::prelude::*;

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

    let animation_ctrl = AnimationCtrl::builder(&panel)
        .with_animation_file("") // Pass empty string for file as we load from bytes
        .with_size(Size::new(100,100))
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

    panel.set_sizer(sizer, true);

    MediaControls {
        panel,
        animation_ctrl,
    }
} 
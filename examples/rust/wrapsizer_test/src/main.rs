use wxdragon::prelude::*;
use wxdragon::sizers::{WrapSizer, WrapSizerFlag};

fn main() {
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("WrapSizer Test")
            .with_size(Size::new(600, 400))
            .build();

        let panel = Panel::builder(&frame).build();

        // Create a main vertical sizer
        let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Add a title
        let title = StaticText::builder(&panel)
            .with_label("WrapSizer Demo - Resize the window to see wrapping")
            .build();
        main_sizer.add(&title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        // Create a WrapSizer with default flags
        let wrap_sizer = WrapSizer::builder(Orientation::Horizontal)
            .with_flags(WrapSizerFlag::DefaultFlags)
            .build();

        // Add some buttons to the wrap sizer
        for i in 1..=12 {
            let button = Button::builder(&panel)
                .with_label(&format!("Button {}", i))
                .build();
            wrap_sizer.add(&button, 0, SizerFlag::All, 5);
        }

        // Add the wrap sizer to the main sizer
        main_sizer.add_sizer(&wrap_sizer, 1, SizerFlag::Expand | SizerFlag::All, 10);

        // Create another section with different flags
        let section_title = StaticText::builder(&panel)
            .with_label("WrapSizer with ExtendLastOnEachLine disabled:")
            .build();
        main_sizer.add(&section_title, 0, SizerFlag::All | SizerFlag::AlignCenterHorizontal, 10);

        let wrap_sizer2 = WrapSizer::builder(Orientation::Horizontal)
            .with_flags(WrapSizerFlag::RemoveLeadingSpaces)
            .build();

        // Add some checkboxes to the second wrap sizer
        for i in 1..=8 {
            let checkbox = CheckBox::builder(&panel)
                .with_label(&format!("Option {}", i))
                .build();
            wrap_sizer2.add(&checkbox, 0, SizerFlag::All, 5);
        }

        main_sizer.add_sizer(&wrap_sizer2, 1, SizerFlag::Expand | SizerFlag::All, 10);

        panel.set_sizer(main_sizer, true);
        frame.show(true);
        frame.centre();
    });
} 
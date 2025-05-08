use wxdragon::prelude::*;

fn main() {
    wxdragon::main(|handle: &mut WxdAppHandle| {
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        let sizer = BoxSizer::builder(VERTICAL).build();

        let button = Button::builder(&frame)
            .with_label("Click me")
            .build();

        button.bind(EventType::COMMAND_BUTTON_CLICKED, |_| {
            println!("Button clicked");
        });

        sizer.add(&button, 1, ALIGN_CENTER_HORIZONTAL | ALIGN_CENTER_VERTICAL, 0);

        frame.set_sizer(sizer, true);

        frame.show(true);
        frame.centre();

        handle.preserve(frame.clone());

        true
    });
}
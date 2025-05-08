# wxDragon

`wxDragon` provides safe, idiomatic Rust bindings for the [wxWidgets](https://www.wxwidgets.org/) cross-platform GUI toolkit. It aims to make GUI development in Rust feel natural and robust, leveraging wxWidgets' extensive set of native-looking controls and features.

This crate is built on top of [`wxdragon-sys`](https://crates.io/crates/wxdragon-sys), which handles the unsafe FFI bindings to a C++ wrapper library (`libwxdragon`) that, in turn, interfaces with wxWidgets.

## Features

*   **Safe Abstractions**: Wraps unsafe C API calls from `wxdragon-sys` into safe Rust structs and methods.
*   **Builder Pattern**: Many widgets can be constructed using a convenient builder pattern.
*   **Event Handling**: (Describe the event handling mechanism once more established, e.g., using closures or specific event traits).
*   **Cross-Platform**: Inherits wxWidgets' ability to create applications with native look and feel on Windows, macOS (Cocoa), and Linux (GTK+).
*   **Growing Widget Set**: Continuously expanding coverage of wxWidgets controls and dialogs.

## Getting Started

Add `wxdragon` to your `Cargo.toml`:

```toml
[dependencies]
wxdragon = "0.1.0" # Replace with the desired version from crates.io
```

## Example

Here's a basic example of creating a simple frame using the `wxdragon::main` entry point:

```rust
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
```

(Note: To run this example, you'd place it in a `main.rs` file within a Cargo project that has `wxdragon` as a dependency and ensure your `wxdragon::prelude` is correctly defined and accessible.)

## License

This crate is licensed under the terms of the MIT license OR the Apache License 2.0 (see `LICENSE-MIT` and `LICENSE-APACHE` files in the workspace root).

## Contributing

Contributions are welcome! Please refer to the `CONTRIBUTING.md` file (if one exists) and the development guidelines in the project's main README. 
# XRC Embedding Example

This example demonstrates wxDragon's `include_xrc!` macro family, which embeds XRC (XML Resource) files directly into the executable at compile time, similar to Rust's `include_bytes!` macro.

## Features Demonstrated

- **Compile-time XRC embedding**: XRC content is embedded into the executable
- **No runtime file dependencies**: No need to distribute XRC files separately
- **Type-safe UI access**: Generated structs provide typed access to UI components
- **Multiple root object types**: Support for Frame, Dialog, and Panel root objects
- **Automatic resource management**: XRC resources are properly managed

## Macros Available

### `include_xrc!`
For Frame root objects:
```rust
wxdragon::include_xrc!("file.xrc", StructName, root = root_name);
```

### `include_xrc_dialog!`
For Dialog root objects:
```rust
wxdragon::include_xrc_dialog!("file.xrc", StructName, root = root_name);
```

### `include_xrc_panel!`
For Panel root objects:
```rust
wxdragon::include_xrc_panel!("file.xrc", StructName, root = root_name);
```

## Generated API

Each macro generates a struct with:

- **Root object field**: Direct access to the loaded XRC object
- **`XRC_DATA` constant**: The embedded XRC content as a string
- **`new()` method**: Creates the UI by loading the embedded XRC
- **`xrc_id()` method**: Helper to get XRC IDs for controls

## Example Usage

```rust
// Define the UI struct from embedded XRC
wxdragon::include_xrc!("ui.xrc", MyUI, root = main_frame);

fn main() {
    wxdragon::main(|_| {
        // Create UI from embedded XRC
        let ui = MyUI::new::<wxdragon::widgets::Frame>(None)
            .expect("Failed to load UI");
        
        // Access the embedded XRC data
        println!("XRC size: {} bytes", MyUI::XRC_DATA.len());
        
        // Find child controls
        if let Some(button) = ui.main_frame.find_child_by_xrc_name::<wxdragon::widgets::Button>("my_button") {
            // Use the button...
        }
        
        ui.main_frame.show(true);
        wxdragon::set_top_window(&ui.main_frame);
    });
}
```

## Benefits

1. **Single executable deployment**: No need to bundle XRC files
2. **Compile-time validation**: XRC files are validated at build time
3. **Type safety**: Generated structs provide compile-time type checking
4. **Performance**: No file I/O at runtime
5. **Versioning**: XRC content is locked to the specific build

## Files in this Example

- `test_frame.xrc` - Frame-based UI definition
- `example_dialog.xrc` - Dialog-based UI definition
- `src/main.rs` - Demonstration of both frame and dialog embedding

Run with: `cargo run -p simple_xrc_test` 
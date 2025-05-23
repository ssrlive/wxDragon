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
wxdragon::include_xrc!("file.xrc", StructName);
```

### `include_xrc_dialog!`
For Dialog root objects:
```rust
wxdragon::include_xrc_dialog!("file.xrc", StructName);
```

### `include_xrc_panel!`
For Panel root objects:
```rust
wxdragon::include_xrc_panel!("file.xrc", StructName);
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
wxdragon::include_xrc!("ui.xrc", MyUI);

fn main() {
    wxdragon::main(|_| {
        // Create UI from embedded XRC
        let ui = MyUI::new();
        
        // Access the embedded XRC data
        println!("XRC size: {} bytes", MyUI::XRC_DATA.len());
        
        ui.button.on_click(|_| {
            // add event logic
        });
        
        ui.main_frame.show(true);
    });
}
```

## Benefits

1. **Single executable deployment**: No need to bundle XRC files
2. **Compile-time validation**: XRC files are validated at build time
3. **Type safety**: Generated structs provide compile-time type checking
4. **Performance**: No file I/O at runtime
5. **Versioning**: XRC content is locked to the specific build

Run with: `cargo run -p simple_xrc_test` 
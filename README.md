# wxDragon - A Rust Wrapper for wxWidgets

This project creates a manually crafted C wrapper around the wxWidgets C++ GUI library. The primary goal is to expose a stable C API that can be consumed by `bindgen` to generate unsafe Rust bindings (`-sys` crate), which are then used to build safe, idiomatic Rust wrappers.

## Screenshot

![Screenshot](https://raw.githubusercontent.com/AllenDang/wxDragon/refs/heads/main/asset/screenshot.png)

## Usage

### Programmatic Widget Creation

Add *wxdragon* to your Cargo.toml.

```rust
use wxdragon::prelude::*;

fn main() {
    wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Hello, World!")
            .with_size(Size::new(300, 200))
            .build();

        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let button = Button::builder(&frame).with_label("Click me").build();

        button.on_click( |_| {
            println!("Button clicked");
        });

        sizer.add(
            &button,
            1,
            SizerFlag::AlignCenterHorizontal | SizerFlag::AlignCenterVertical,
            0,
        );

        frame.set_sizer(sizer, true);

        frame.show(true);
        frame.centre();
    });
}
```

### XRC-Based UI Development

wxDragon also supports XRC (XML Resource) files for declarative UI development with compile-time type safety.

You could use [wxFormBuilder](https://github.com/wxFormBuilder/wxFormBuilder) as UI designer to generate XRC.

**1. Define your UI in XRC format (`ui/main.xrc`):**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<resource>
  <object class="wxFrame" name="main_frame">
    <title>wxDragon XRC Demo</title>
    <size>400,300</size>
    <object class="wxPanel" name="main_panel">
      <object class="wxButton" name="hello_button">
        <label>Click Me!</label>
        <pos>50,50</pos>
      </object>
      <object class="wxTextCtrl" name="input_field">
        <value>Enter text here...</value>
        <pos>50,100</pos>
        <size>200,25</size>
      </object>
      <object class="wxStaticText" name="status_label">
        <label>Ready</label>
        <pos>50,150</pos>
      </object>
    </object>
  </object>
</resource>
```

**2. Use the `include_xrc!` macro to generate a typed UI struct:**
```rust
use wxdragon::prelude::*;

// Generate MyUI struct with typed fields for all named widgets
wxdragon::include_xrc!("ui/main.xrc", MyUI);

fn main() {
    wxdragon::main(|_| {
        // Create UI instance - automatically loads XRC and finds all widgets
        let ui = MyUI::new(None);
        
        // Access widgets with full type safety
        let button = &ui.hello_button;      // Button
        let input = &ui.input_field;        // TextCtrl  
        let label = &ui.status_label;       // StaticText
        let frame = &ui.main_frame;         // Frame (root object)
        
        // Bind events with closures
        let label_clone = label.clone();
        let input_clone = input.clone();
        button.on_click(move |_| {
            let text = input_clone.get_value();
            label_clone.set_label(&format!("You entered: {}", text));
            println!("Button clicked! Input: {}", text);
        });
        
        // Show the window
        frame.show(true);
        frame.centre();
    });
}
```

**Key benefits of the XRC approach:**
- **Declarative UI**: Define layouts in XML, separate from logic
- **Compile-time safety**: Auto-generated structs with typed widget fields  
- **No verbose syntax**: Just `MyUI::new()` instead of `MyUI::new::<Frame>()`
- **Designer support**: XRC files can be created with visual designers
- **Automatic widget discovery**: Macro finds all named widgets and generates appropriate Rust types

## Supported Platforms

| Platform | Support                                                      |
| -------- | ------------------------------------------------------------ |
| macOS    | Static Build + Cross build to Windows via gnu                |
| Windows  | Not yet (I don't have a windows machine, need help to update the build.rs to support) |
| Linux    | Not yet (I don't have a linux machine, need help to update the build.rs to support) |

## Approach

1.  **C API:** Define a C API (`rust/wxdragon-sys/cpp/include/wxdragon.h`) using opaque pointers for wxWidgets objects and C functions to interact with them. Uses stable C types for flags/constants and `const char*` for strings.
2.  **C++ Implementation:** Implement the C API functions (`rust/wxdragon-sys/cpp/src/*.cpp`), translating C calls into wxWidgets C++ calls. Manages wxWidgets object creation (`new`) and destruction (`Destroy()`).
3.  **Event Handling:** Implement a robust event handling system using wxWidgets' native mechanism:
    *   **C++ Side:** Uses `wxEvtHandler::Bind` with a C++ functor (`CxxClosureVoid`) that wraps the Rust trampoline function pointer and the Rust closure data pointer. The lifetime of this functor (and thus the Rust closure `Box`) is managed by wxWidgets.
    *   **C API:** Exposes `wxd_EvtHandler_Bind`, which takes a stable C enum value (`WXDEventTypeCEnum`) representing the event type, a C function pointer (to the Rust trampoline), and a `void*` (the Rust closure box pointer). The C++ implementation maps the stable C enum back to the appropriate `wxEVT_XXX` constant for the call to `Bind`.
    *   **Rust Side:** Provides a `WxEvtHandler` trait with a safe `bind` method. Uses a type-safe `EventType` enum wrapping the stable `WXDEventTypeCEnum` from the FFI layer. Closures (`FnMut(Event) + 'static`) are used for callbacks. The `Drop` implementation in the C++ functor calls a Rust function (`drop_rust_closure_box`) to correctly free the closure `Box` when the binding is destroyed.
4.  **Build:** Use CMake to build the C wrapper library (invoked via `wxdragon-sys/build.rs` from `rust/wxdragon-sys/cpp/CMakeLists.txt`). wxWidgets source is downloaded and built automatically as part of this process.
5.  **Rust Bindings:** Use `bindgen` on the C header (`rust/wxdragon-sys/cpp/include/wxdragon.h`) to generate a `wxdragon-sys` crate.
6.  **Safe Rust Wrapper:** Develop a `wxdragon` Rust crate providing safe abstractions over the `wxdragon-sys` crate.
7.  **Incremental:** Started with core widgets (`wxApp`, `wxFrame`) and expand coverage gradually.
8.  **Constant Handling:**
    *   **Event Types:** Event types (`wxEVT_*`) are handled via a manually defined stable C enum (`WXDEventTypeCEnum` in `rust/wxdragon-sys/cpp/include/wxdragon.h`) and a corresponding Rust enum (`EventType` in `rust/wxdragon/src/event.rs`). This ensures stability across wxWidgets versions and platforms for event type identifiers.
    *   **Other Constants (Styles, IDs, Flags):** For other constants like style flags, standard IDs, etc., wxDragon now uses **pre-generated Rust files**. These files (e.g., `wx_msw_constants.rs`, `wx_gtk_constants.rs`, `wx_osx_constants.rs`) are located in `rust/wxdragon-sys/src/generated_constants/` and are checked into the repository. The `const_extractor` tool (in `tools/const_extractor/`) is used by maintainers to generate these files for each major platform/wxWidgets port. During a user's build (`cargo build`), the `wxdragon-sys/build.rs` script detects the target OS and copies the appropriate pre-generated file to `$OUT_DIR/wx_other_constants.rs`. This approach enhances cross-compilation capabilities and simplifies the build process for users by removing the need to run `const_extractor` at build time.

## Project Structure

```
wxdragon/
├── rust/
│   ├── wxdragon-sys/    # Raw FFI bindings, C++ source, and build script
│   │   ├── cpp/             # C++ wrapper source, headers, and CMake file
│   │   │   ├── include/     # Public C header (wxdragon.h)
│   │   │   ├── src/         # C++ wrapper implementation
│   │   │   ├── tools/       # Helper tools (e.g., const_extractor for maintainers)
│   │   │   │   └── const_extractor/
│   │   │   └── CMakeLists.txt # CMake for libwxdragon & wxWidgets
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── generated_constants/ # Pre-generated OS-specific constants
│   │   ├── build.rs         # Rust build script
│   │   └── Cargo.toml
│   ├── wxdragon/        # Safe Rust wrapper
│   │   ├── src/
│   │   └── Cargo.toml
│   └── Cargo.toml         # Workspace Cargo.toml (might be at root)
├── examples/
│   └── rust/
│       └── gallery/
├── Cargo.toml           # Top-level Workspace Cargo.toml (if not in rust/)
└── README.md            # This file
```
*(Note: The `const_extractor` path and the exact location of the workspace `Cargo.toml` in the diagram might need slight adjustment based on your final layout.)*

## Build Instructions

1.  Install a C++ compiler suitable for your platform (CMake is used by the build script but doesn't need separate installation if you have `cmake` in your PATH or use the `cmake` crate feature that bundles it).
2.  **wxWidgets is downloaded and built automatically as part of the project.**
3.  Navigate to the root of the repository (or the directory containing the main workspace `Cargo.toml`).
4.  Run `cargo build` or `cargo run -p gallery`.
    *   The `wxdragon-sys` build script (`build.rs`) will automatically:
        *   Download the wxWidgets source tarball (version 3.2.8 currently) if not already present.
        *   Extract wxWidgets.
        *   Invoke CMake to configure and build:
            *   The wxWidgets static libraries.
            *   The C++ wrapper static library (`libwxdragon.a`).
        *   Copy the appropriate pre-generated platform-specific constants file (e.g., `wx_msw_constants.rs`) from `rust/wxdragon-sys/src/generated_constants/` to `$OUT_DIR/wx_other_constants.rs`.
        *   Generate Rust FFI bindings (`bindings.rs`) using `bindgen` against `rust/wxdragon-sys/cpp/include/wxdragon.h` and the built wxWidgets headers.
        *   Configure Cargo to link `libwxdragon.a` and the necessary wxWidgets libraries.
        *   **For macOS:** Uses a hardcoded set of linker flags and bindgen include paths derived from `wx-config` for stability.
        *   **For Linux/Windows:** Current build script has placeholders; full support for these platforms (e.g., via `pkg-config` on Linux) is pending.

## Cross-Compilation (macOS to Windows)

To build the project on macOS targeting Windows (specifically `x86_64-pc-windows-gnu`):

1.  **Install Dependencies via Homebrew:**
    ```bash
    brew install mingw-w64
    ```
    This installs the MinGW-w64 toolchain, including the necessary C/C++ cross-compilers (e.g., `x86_64-w64-mingw32-gcc`, `x86_64-w64-mingw32-g++`) and linker.

2.  **Add Rust Target:**
    ```bash
    rustup target add x86_64-pc-windows-gnu
    ```
    This downloads the Rust standard library pre-compiled for the target.

3.  **Build:**
    ```bash
    cargo build --target=x86_64-pc-windows-gnu
    ```
    Or for a release build:
    ```bash
    cargo build --target=x86_64-pc-windows-gnu --release
    ```
    The `wxdragon-sys/build.rs` script contains specific logic to detect this target, configure CMake appropriately, and set the correct linker flags (including static linking for `libstdc++` and `libgcc`) to produce a standalone `.exe` file.

## Current Status (Reflecting Recent Build System Changes)

*   Basic project structure set up with CMake and Cargo workspace.
*   Core C wrapper implementation for various widgets is ongoing.
*   **Build System:**
    *   [x] wxWidgets source (currently 3.2.8) is automatically **downloaded, extracted, configured, and built statically** by `wxdragon-sys/build.rs` using the `cmake` crate.
    *   [x] C++ wrapper library (`libwxdragon.a`) built by CMake invoked from `build.rs` (using `rust/wxdragon-sys/cpp/CMakeLists.txt`).
    *   [x] **Hybrid Constant Handling:**
        *   Event Types: Stable C enum (`WXDEventTypeCEnum`) and Rust enum (`EventType`).
        *   Other Constants: Pre-generated platform-specific Rust files (e.g., `wx_msw_constants.rs`, `wx_gtk_constants.rs`) located in `rust/wxdragon-sys/src/generated_constants/`, copied to `$OUT_DIR/wx_other_constants.rs` by `build.rs`.
    *   [x] `build.rs` supports incremental C++ builds.
    *   [x] **macOS Build:** Uses hardcoded linker and bindgen flags for stability.
    *   [ ] **Linux/Windows Build:** Automated build support is under development.
*   **`wxdragon-sys` Rust crate:**
    *   [x] `bindgen` generates raw FFI bindings from `rust/wxdragon-sys/cpp/include/wxdragon.h`.
    *   [x] `build.rs` correctly invokes CMake, downloads wxWidgets, copies pre-generated constants, and links libraries using a manual configuration for macOS.
*   **`wxdragon` Safe Rust Wrapper:**
    *   [x] Safe wrappers for core widgets, sizers, and menu components implemented with builder pattern.
    *   [x] **Stable Event Handling:**
        *   `WxEvtHandler` trait provides generic `bind(EventType, FnMut(Event))` using the stable `EventType` enum.
        *   Relies on C++ `wxEvtHandler::Bind` with functor for lifetime management.
    *   [x] Safe application entry point (`wxdragon::main`).
    *   [x] Basic lifetime management (`Drop`, `preserve()`).
    *   [x] Basic layout management (`set_sizer`, `set_sizer_and_fit`).
    *   [x] Gallery Rust example (`gallery`) demonstrates core features and event handling.

## TODO List - Implementation Tasks

*   **Core:**
    *   [x] `wxApp`
    *   [x] `wxFrame`
*   **Widgets/Controls:**
    *   **Basic:**
        *   [x] `wxButton`
        *   [x] `wxStaticText`
        *   [x] `wxTextCtrl`
        *   [x] `wxCheckBox`
        *   [x] `wxRadioButton`
        *   [x] `wxToggleButton`
        *   [x] `wxGauge`
        *   [x] `wxSlider`
        *   [x] `wxSpinCtrl`
        *   [x] `wxSpinButton`
        *   [x] `wxDatePickerCtrl`
        *   [x] `wxBitmapButton`
        *   [x] `wxRadioBox`
        *   [x] `wxScrollBar`
        *   [x] `wxSpinCtrlDouble`
        *   [x] `wxStaticBitmap`
        *   [x] `wxStaticLine`
        *   [x] `wxSearchCtrl`
        *   [ ] `wxStyledTextCtrl`
        *   [ ] `wxRichTextCtrl`
        *   [x] `wxHyperlinkCtrl`
        *   [x] `wxActivityIndicator`
        *   [x] `wxAnimationCtrl`
        *   [x] `wxCommandLinkButton`
    *   **Choices/Lists:**
        *   [x] `wxChoice`
        *   [x] `wxComboBox`
        *   [x] `wxListBox`
        *   [x] `wxCheckListBox`
        *   [x] `wxTreeCtrl`
        *   [x] `wxListCtrl`
        *   [x] `wxBitmapComboBox`
        *   [ ] `wxComboCtrl`
        *   [x] `wxDataViewCtrl`
        *   [x] `wxDataViewListCtrl`
        *   [x] `wxDataViewTreeCtrl`
        *   [x] `wxEditableListBox`
        *   [x] `wxFileCtrl`
        *   [ ] `wxGenericDirCtrl`
        *   [ ] `wxHtmlListBox`
        *   [ ] `wxOwnerDrawnComboBox`
        *   [ ] `wxPropertyGrid`
        *   [ ] `wxPropertyGridManager`
        *   [x] `wxRearrangeCtrl`
        *   [ ] `wxSimpleHtmlListBox`
        *   [ ] `wxTreeListCtrl`
    *   **Picker Controls:**
        *   [x] `wxColourPickerCtrl`
        *   [x] `wxDatePickerCtrl`
        *   [x] `wxFilePickerCtrl`
        *   [x] `wxDirPickerCtrl`
        *   [x] `wxFontPickerCtrl`
        *   [x] `wxTimePicker`
    *   **Containers:**
        *   [x] `wxPanel`
        *   [x] `wxScrolledWindow`
        *   [x] `wxSplitterWindow`
        *   [x] `wxNotebook` (Tabs)
        *   [x] `wxStaticBox`
        *   [x] `wxAuiMDIChildFrame`
        *   [x] `wxAuiMDIParentFrame`
        *   [x] `wxAuiMDIClientWindow`
        *   [x] `wxAuiNotebook`
        *   [x] `wxAuiToolBar`
        *   [ ] `wxBannerWindow`
        *   [ ] `wxChoicebook`
        *   [ ] `wxCollapsiblePane`
        *   [ ] `wxListbook`
        *   [ ] `wxSimplebook`
        *   [x] `wxTreebook`
    *   **Other:**
        *   [x] `wxArtProvider` (*Basic support for `GetBitmap`*)
        *   [x] `wxCalendarCtrl`
        *   [ ] `wxGLCanvas`
        *   [ ] `wxHtmlWindow`
        *   [x] `wxMediaCtrl`
        *   [x] `wxNotificationMessage`
        *   [ ] `wxRichToolTip`
        *   [ ] `wxSplashScreen`
        *   [x] `wxStatusBar`
        *   [ ] `wxTaskBarIcon`
        *   [ ] `wxTimer`
        *   [ ] `wxWebView`
        *   [ ] `wxWizard`
*   **Sizers:**
    *   [x] `wxBoxSizer`
    *   [x] Expand `BoxSizer` methods (`AddSizer`, `AddSpacer`, `AddStretchSpacer`)
    *   [ ] `wxGridSizer`
    *   [x] `wxFlexGridSizer`
    *   [x] `wxStaticBoxSizer`
*   **Menus & Toolbars:**
    *   [x] `wxMenuBar`
    *   [x] `wxMenu`
    *   [x] `wxMenuItem`
    *   [x] `wxToolBar` (*Use `Frame::create_tool_bar` and call `realize()` after adding tools*)
*   **Dialogs:**
    *   [x] `wxDialog` (base class)
    *   [x] `wxMessageDialog`
    *   [x] `wxFileDialog`
    *   [x] `wxDirDialog`
    *   [x] `wxColourDialog`
    *   [x] `wxFontDialog`
    *   [x] `wxTextEntryDialog`
    *   [x] `wxProgressDialog`
    *   [x] `wxSingleChoiceDialog`
    *   [x] `wxMultiChoiceDialog`
    *   [ ] `wxSplashScreen` (*Moved here, also related to Other*)
    *   [ ] `wxWizard` (*Moved here, also related to Other*)
*   **Event Handling Expansion:**
    *   [x] Add specific event data access (e.g., `event.get_string()`, `event.get_position()`, `event.get_checked()`, `event.get_key_code()`).
*   **XRC Support:**
    *   [x] Implement C++ bindings for `wxXmlResource` in `wxdragon-sys`.
    *   [x] Create safe Rust wrappers for `wxXmlResource` in `wxdragon`.
    *   [x] Develop `include_xrc!("file.xrc", UStructName)` procedural macro to generate Rust UI structs from XRC files.
        *   [x] Compile-time XRC parsing.
        *   [x] Mapping XRC widget classes to `wxDragon` types.
        *   [x] Generation of struct with typed fields for named XRC widgets.
        *   [x] Automatic loading of XRC and widget retrieval in the generated struct's constructor.
*   **Documentation:**
    *   [ ] Improve C API docs (doxygen?).
    *   [ ] Rust API docs (rustdoc).
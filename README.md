# wxDragon - A Rust Wrapper for wxWidgets

This project creates a manually crafted C wrapper around the wxWidgets C++ GUI library. The primary goal is to expose a stable C API that can be consumed by `bindgen` to generate unsafe Rust bindings (`-sys` crate), which are then used to build safe, idiomatic Rust wrappers.

## Screenshot

![Screenshot](https://raw.githubusercontent.com/AllenDang/wxDragon/refs/heads/main/asset/screenshot.png)

## Usage

Add *wxdragon* to your Cargo.toml.

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
│       └── minimal_rust/
├── Cargo.toml           # Top-level Workspace Cargo.toml (if not in rust/)
└── README.md            # This file
```
*(Note: The `const_extractor` path and the exact location of the workspace `Cargo.toml` in the diagram might need slight adjustment based on your final layout.)*

## Build Instructions

1.  Install a C++ compiler suitable for your platform (CMake is used by the build script but doesn't need separate installation if you have `cmake` in your PATH or use the `cmake` crate feature that bundles it).
2.  **wxWidgets is downloaded and built automatically as part of the project.**
3.  Navigate to the root of the repository (or the directory containing the main workspace `Cargo.toml`).
4.  Run `cargo build` or `cargo run -p minimal_rust`.
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
    *   [x] Minimal Rust example (`minimal_rust`) demonstrates core features and event handling.

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
        *   [ ] `wxAnimationCtrl`
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
        *   [ ] `wxDataViewCtrl`
        *   [ ] `wxDataViewListCtrl`
        *   [ ] `wxDataViewTreeCtrl`
        *   [ ] `wxEditableListBox`
        *   [ ] `wxFileCtrl`
        *   [ ] `wxGenericDirCtrl`
        *   [ ] `wxHtmlListBox`
        *   [ ] `wxOwnerDrawnComboBox`
        *   [ ] `wxPropertyGrid`
        *   [ ] `wxPropertyGridManager`
        *   [ ] `wxRearrangeCtrl`
        *   [ ] `wxSimpleHtmlListBox`
        *   [ ] `wxTreeListCtrl`
    *   **Picker Controls:**
        *   [x] `wxColourPickerCtrl`
        *   [x] `wxDatePickerCtrl`
        *   [ ] `wxFilePickerCtrl`
        *   [ ] `wxDirPickerCtrl`
        *   [ ] `wxFontPickerCtrl`
    *   **Containers:**
        *   [x] `wxPanel`
        *   [x] `wxScrolledWindow`
        *   [x] `wxSplitterWindow`
        *   [x] `wxNotebook` (Tabs)
        *   [x] `wxStaticBox`
        *   [ ] `wxAuiMDIChildFrame`
        *   [ ] `wxAuiMDIClientWindow`
        *   [ ] `wxAuiMDIParentFrame`
        *   [ ] `wxAuiNotebook`
        *   [ ] `wxBannerWindow`
        *   [ ] `wxChoicebook`
        *   [ ] `wxCollapsiblePane`
        *   [ ] `wxListbook`
        *   [ ] `wxSimplebook`
        *   [x] `wxTreebook`
    *   **Other:**
        *   [x] `wxArtProvider` (*Basic support for `GetBitmap`*)
        *   [ ] `wxAuiToolBar`
        *   [x] `wxCalendarCtrl`
        *   [ ] `wxGLCanvas`
        *   [ ] `wxHtmlWindow`
        *   [ ] `wxMediaCtrl`
        *   [ ] `wxNotificationMessage`
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
    *   [ ] `wxColourDialog`
    *   [ ] `wxFontDialog`
    *   [ ] `wxTextEntryDialog`
    *   [ ] `wxProgressDialog`
    *   [ ] `wxSplashScreen` (*Moved here, also related to Other*)
    *   [ ] `wxWizard` (*Moved here, also related to Other*)
*   **Event Handling Expansion:**
    *   [x] Add specific event data access (e.g., `event.get_string()`, `event.get_position()`, `event.get_checked()`, `event.get_key_code()`).
*   **Refinements:**
    *   [ ] Thread safety considerations (e.g., sending events between threads).
*   **Documentation:**
    *   [ ] Improve C API docs (doxygen?).
    *   [ ] Rust API docs (rustdoc).

# Guideline for wxDragon Development:

When implementing any new feature (widget, sizer, event, etc.), prioritize safety and consistency across all layers, ensuring a clean separation of concerns and performing incremental build checks:

## Constant Handling:
. **Event Types:**
    . If a new event binding is needed, add a new variant to the `WXDEventTypeCEnum` in `rust/wxdragon-sys/cpp/include/wxdragon.h` (assigning a **stable** integer value).
    . Update the `switch` statement in `rust/wxdragon-sys/cpp/src/event.cpp`'s `wxd_EvtHandler_Bind` function to map it to the corresponding `wxEVT_XXX` constant.
    . Add a corresponding constant to the Rust `EventType` enum in `rust/wxdragon/src/event.rs`.
. **Other Constants (Styles, IDs, Flags) - For Maintainers:**
    . Identify needed `wxXXX` constants (e.g., `wxTAB_TRAVERSAL`, `wxLB_SORT`, `wxID_OK`).
    . Add their C++ names (e.g., "wxTAB_TRAVERSAL", "wxLB_SORT") to the `constants_to_extract` vector in `rust/wxdragon-sys/cpp/tools/const_extractor/main.cpp`.
    . **Generation Process (Maintainer Task):**
        1. For each target platform (e.g., Windows, Linux, macOS), build wxWidgets and the `const_extractor` tool for that platform.
        2. Run the compiled `const_extractor` against that platform's wxWidgets build.
        3. Save the output into the corresponding platform-specific file in `rust/wxdragon-sys/src/generated_constants/` (e.g., `wx_msw_constants.rs`, `wx_gtk_constants.rs`). These files should generate `pub const WXD_XXX` constants.
        4. Commit these updated pre-generated files to the repository.
    . **Usage in Safe Wrapper:** Use the `WXD_XXX` constants (which will be available via `wxdragon_sys::WXD_XXX` after `build.rs` copies the correct file) in the safe Rust wrapper, typically by defining local constants within the relevant widget module (e.g., `pub const TAB_TRAVERSAL: i64 = wxdragon_sys::WXD_TAB_TRAVERSAL;` in `panel.rs`).
. **Build Check (During Development of Constant Generation):** When modifying `const_extractor` or the generation process, verify constants are generated correctly into the platform-specific files and that `build.rs` copies the correct one to `$OUT_DIR/wx_other_constants.rs` making them accessible via `wxdragon_sys::WXD_XXX`.

## C API (`rust/wxdragon-sys/cpp/include/wxdragon.h`):
. Define the minimal C interface (opaque pointers, C types, `const char*`).
. **Events:** Use the existing `wxd_EvtHandler_Bind`. Add C functions (e.g., `wxd_CommandEvent_GetString`) if specific event data needs accessing from Rust.
. Keep function signatures C-idiomatic.
. **Build Check:** `cd rust && cargo build` (or `cargo build -p wxdragon-sys`). Ensure `bindgen` can parse `rust/wxdragon-sys/cpp/include/wxdragon.h` correctly and `wxdragon-sys` compiles.

## C++ Implementation (`rust/wxdragon-sys/cpp/src/*.cpp`):
. Implement the C functions defined in `rust/wxdragon-sys/cpp/include/wxdragon.h`.
. Translate C calls directly to the corresponding wxWidgets C++ calls...
. **Build Check:** `cd rust && cargo build` (or `cargo build -p wxdragon-sys`). This will trigger CMake via `wxdragon-sys/build.rs` to compile the C++ code. Fix any C++ compilation or linking errors.

## Rust Safe Wrapper (`rust/wxdragon/src`):
. (Unchanged)

## Guideline for Widget Builder Pattern in wxDragon:
. (Unchanged)

## Guideline for Locating CMake Artifacts in wxDragon Project:
. (This section might need review/update as `$OUT_DIR/build/build` is specific to the old structure. Now it's more like `$OUT_DIR/build/wxwidgets_build` for wx artifacts and `$OUT_DIR/build/lib` for `libwxdragon.a`.)
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target = env::var("TARGET").unwrap();

    // --- 1. Generate FFI Bindings ---
    println!("info: Generating FFI bindings...");

    let mut bindings_builder = bindgen::Builder::default()
        .header("cpp/include/wxdragon.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Platform-specific bindgen configuration
    if target_os == "macos" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXOSX_COCOA__")
            .clang_arg("-D__WXMAC__")
            .clang_arg("-D__WXOSX__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DNDEBUG");
    } else if target_os == "windows" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXMSW__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DwxUSE_UNICODE=1")
            .clang_arg("-DNDEBUG");
    } else if target_os == "linux" {
        bindings_builder = bindings_builder
            .clang_arg("-D__WXGTK__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DNDEBUG");
    }

    // Add feature flags for conditional compilation
    if cfg!(feature = "aui") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_AUI=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_AUI=0");
    }

    if cfg!(feature = "media-ctrl") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_MEDIACTRL=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_MEDIACTRL=0");
    }

    if cfg!(feature = "webview") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_WEBVIEW=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_WEBVIEW=0");
    }

    if cfg!(feature = "stc") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_STC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_STC=0");
    }

    if cfg!(feature = "xrc") {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_XRC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DWXD_USE_XRC=0");
    }

    bindings_builder = bindings_builder.clang_arg(format!("--target={}", target));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("info: Successfully generated FFI bindings");

    // Skip library setup for docs.rs and rust-analyzer
    if env::var("DOCS_RS").is_ok() || env::var("RUST_ANALYZER") == Ok("true".to_string()) {
        return;
    }

    // --- 2. Download and Setup Pre-built Libraries ---
    let wx_version = "3.3.0";

    download_prebuilt_libraries(wx_version, &out_dir, &target_os, &target_env)
        .expect("Failed to download pre-built wxWidgets libraries");

    // --- 3. Build wxDragon Wrapper ---
    build_wxdragon_wrapper(&out_dir, &target_os, &target_env)
        .expect("Failed to build wxDragon wrapper library");

    // --- 4. Setup Linking ---
    setup_linking(&target_os, &target_env, &out_dir);
}

fn download_prebuilt_libraries(
    wx_version: &str,
    out_dir: &Path,
    target_os: &str,
    target_env: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => "wxwidgets-linux-x64",
        ("macos", "x86_64", _) => "wxwidgets-macos-x64",
        ("macos", "aarch64", _) => "wxwidgets-macos-arm64",
        ("windows", "x86_64", "msvc") => "wxwidgets-windows-msvc-x64",
        ("windows", "x86_64", "gnu") => "wxwidgets-windows-gnu-x64",
        _ => {
            return Err(format!(
                "Unsupported target platform: {}-{}-{}",
                target_os, target_arch, target_env
            )
            .into())
        }
    };

    let download_url = format!(
        "https://github.com/AllenDang/wxDragon/releases/download/wxwidgets-{}/{}.tar.gz",
        wx_version, artifact_name
    );

    let tarball_dest_path = out_dir.join(format!("{}.tar.gz", artifact_name));
    let extracted_path = out_dir.join(artifact_name);

    // Skip download if already extracted
    if extracted_path.exists() {
        println!(
            "info: Using cached pre-built libraries at {:?}",
            extracted_path
        );
        return Ok(());
    }

    println!(
        "info: Downloading pre-built libraries from: {}",
        download_url
    );

    // Download the pre-built libraries
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .expect("Failed to build reqwest client");

    let resp = client
        .get(&download_url)
        .send()
        .map_err(|e| format!("Failed to download {}: {}", download_url, e))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Failed to download {}: HTTP {}",
            download_url,
            resp.status()
        )
        .into());
    }

    // Save the tarball
    let mut out_file = File::create(&tarball_dest_path).map_err(|e| {
        format!(
            "Failed to create destination file {:?}: {}",
            tarball_dest_path, e
        )
    })?;
    let content = resp
        .bytes()
        .map_err(|e| format!("Failed to read response content: {}", e))?;
    std::io::copy(&mut content.as_ref(), &mut out_file)
        .map_err(|e| format!("Failed to write downloaded content: {}", e))?;

    println!(
        "info: Downloaded pre-built libraries to {:?}",
        tarball_dest_path
    );

    // Extract the tarball
    let tarball_file = File::open(&tarball_dest_path)
        .map_err(|e| format!("Failed to open tarball {:?}: {}", tarball_dest_path, e))?;
    let gz_decoder = flate2::read::GzDecoder::new(tarball_file);
    let mut archive = tar::Archive::new(gz_decoder);

    archive.unpack(&out_dir).map_err(|e| {
        format!(
            "Failed to extract {} to {:?}: {}",
            artifact_name, out_dir, e
        )
    })?;

    if !extracted_path.exists() {
        return Err(format!(
            "Extraction did not result in expected directory: {:?}. Check tarball structure.",
            extracted_path
        )
        .into());
    }

    println!(
        "info: Successfully extracted pre-built libraries to {:?}",
        extracted_path
    );
    Ok(())
}

fn setup_linking(target_os: &str, target_env: &str, out_dir: &Path) {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Get the pre-built library directory
    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => "wxwidgets-linux-x64",
        ("macos", "x86_64", _) => "wxwidgets-macos-x64",
        ("macos", "aarch64", _) => "wxwidgets-macos-arm64",
        ("windows", "x86_64", "msvc") => "wxwidgets-windows-msvc-x64",
        ("windows", "x86_64", "gnu") => "wxwidgets-windows-gnu-x64",
        _ => panic!(
            "Unsupported target platform: {}-{}-{}",
            target_os, target_arch, target_env
        ),
    };

    let lib_dir = out_dir.join(artifact_name);

    // Add library search path
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // Link wxdragon wrapper library (will be built separately or included in pre-built package)
    println!("cargo:rustc-link-lib=static=wxdragon");

    // Platform-specific library linking
    match target_os {
        "macos" => link_macos_libraries(),
        "windows" => link_windows_libraries(target_env),
        "linux" => link_linux_libraries(),
        _ => panic!("Unsupported target OS: {}", target_os),
    }
}

fn link_macos_libraries() {
    // Core wxWidgets libraries for macOS
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_core-3.3");
    println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_adv-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_gl-3.3");
    println!("cargo:rustc-link-lib=static=wx_osx_cocoau_propgrid-3.3");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_aui-3.3");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_media-3.3");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_webview-3.3");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.3");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_stc-3.3");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_xrc-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }

    // Third-party libraries
    println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
    println!("cargo:rustc-link-lib=static=wxpng-3.3");
    println!("cargo:rustc-link-lib=static=wxtiff-3.3");
    println!("cargo:rustc-link-lib=static=wxregexu-3.3");

    // System libraries
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=iconv");
    println!("cargo:rustc-link-lib=c++");

    // STC-specific libraries
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
        println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
    }

    // macOS frameworks
    println!("cargo:rustc-link-lib=framework=AudioToolbox");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");

    // Conditional frameworks for macOS
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=AVKit");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
    }
}

fn link_windows_libraries(target_env: &str) {
    // Check if this is cross-compilation from macOS to Windows GNU
    let is_macos_to_windows_gnu = cfg!(target_os = "macos") && target_env == "gnu";

    // For Windows GNU (both native and cross-compilation), use the actual library names from pre-built packages
    // Core wxWidgets libraries
    println!("cargo:rustc-link-lib=static=wxmsw33u_core");
    println!("cargo:rustc-link-lib=static=wxmsw33u_adv");
    println!("cargo:rustc-link-lib=static=wxbase33u");
    println!("cargo:rustc-link-lib=static=wxmsw33u_gl");
    println!("cargo:rustc-link-lib=static=wxmsw33u_propgrid");
    println!("cargo:rustc-link-lib=static=wxmsw33u_html");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_aui");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_media");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_webview");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_stc");
        println!("cargo:rustc-link-lib=static=wxscintilla");
        println!("cargo:rustc-link-lib=static=wxlexilla");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wxmsw33u_xrc");
        println!("cargo:rustc-link-lib=static=wxbase33u_xml");
    }

    // Third-party libraries (using the actual names from pre-built packages)
    println!("cargo:rustc-link-lib=static=wxtiff");
    println!("cargo:rustc-link-lib=static=wxjpeg");
    println!("cargo:rustc-link-lib=static=wxpng");
    println!("cargo:rustc-link-lib=static=wxregexu");
    println!("cargo:rustc-link-lib=static=wxzlib");
    println!("cargo:rustc-link-lib=static=wxexpat");

    // Runtime libraries
    if target_env == "gnu" {
        if is_macos_to_windows_gnu {
            println!("info: Using static linking for cross-compilation from macOS to Windows GNU");

            // --- Dynamically find MinGW GCC library paths ---
            let gcc_path = "x86_64-w64-mingw32-gcc"; // Assume it's in PATH

            // Find the path containing libgcc.a
            let output_libgcc = std::process::Command::new(gcc_path)
                .arg("-print-libgcc-file-name")
                .output();

            if let Ok(output) = output_libgcc {
                if output.status.success() {
                    let libgcc_path_str =
                        String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !libgcc_path_str.is_empty() {
                        let libgcc_path = std::path::Path::new(&libgcc_path_str);
                        if let Some(libgcc_dir) = libgcc_path.parent() {
                            println!("cargo:rustc-link-search=native={}", libgcc_dir.display());
                            println!(
                                "info: Added GCC library search path (from libgcc): {}",
                                libgcc_dir.display()
                            );

                            // Attempt to find the path containing libstdc++.a (often one level up, in `../<target>/lib`)
                            if let Some(gcc_dir) = libgcc_dir.parent() {
                                // e.g., .../gcc/x86_64-w64-mingw32/15.1.0 -> .../gcc/x86_64-w64-mingw32
                                if let Some(toolchain_lib_dir) = gcc_dir.parent() {
                                    // e.g., .../gcc/x86_64-w64-mingw32 -> .../gcc
                                    if let Some(base_lib_dir) = toolchain_lib_dir.parent() {
                                        // e.g., .../gcc -> .../lib
                                        // Construct the expected path for libstdc++.a based on structure
                                        let libstdcpp_dir = base_lib_dir
                                            .parent()
                                            .unwrap()
                                            .join("x86_64-w64-mingw32/lib"); // ../../x86_64-w64-mingw32/lib
                                        if libstdcpp_dir.exists() && libstdcpp_dir != libgcc_dir {
                                            println!(
                                                "cargo:rustc-link-search=native={}",
                                                libstdcpp_dir.display()
                                            );
                                            println!(
                                                "info: Added GCC library search path (for libstdc++): {}",
                                                libstdcpp_dir.display()
                                            );
                                        } else {
                                            println!("info: Could not find or verify expected libstdc++ path relative to libgcc path: {}", libstdcpp_dir.display());
                                        }
                                    }
                                }
                            }
                        } else {
                            println!(
                                "cargo:warning=Could not get parent directory from libgcc path: {}",
                                libgcc_path_str
                            );
                        }
                    } else {
                        println!(
                            "cargo:warning=Command -print-libgcc-file-name returned empty output."
                        );
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!(
                        "cargo:warning=Failed to run '{} -print-libgcc-file-name': {}",
                        gcc_path, stderr
                    );
                    println!("cargo:warning=Static linking for stdc++/gcc might fail. Falling back to hoping they are in default paths.");
                }
            } else {
                println!("cargo:warning=Could not execute x86_64-w64-mingw32-gcc. Static linking for stdc++/gcc might fail.");
            }
            // --- End dynamic path finding ---

            // Static linking for cross-compilation to avoid runtime dependencies
            println!("cargo:rustc-link-lib=static=stdc++");
            println!("cargo:rustc-link-lib=static=gcc");
            println!("cargo:rustc-link-lib=static=gcc_eh");
            println!("cargo:rustc-link-lib=static=pthread");
            // Add linker arguments for fully static C++ runtime
            println!("cargo:rustc-link-arg=-static-libgcc");
            println!("cargo:rustc-link-arg=-static-libstdc++");

            // Use UCRT instead of MSVCRT for cross-compilation (modern MinGW standard)
            // This is critical for compatibility with GCC 15.1.0 and recent MinGW distributions
            println!("cargo:rustc-link-lib=ucrt");
            println!("info: Using UCRT runtime for cross-compilation compatibility");
        } else {
            // Native Windows GNU builds
            // Check if we're in MSYS2 environment which uses UCRT
            let in_msys2 = env::var("MSYSTEM").is_ok() || 
                           env::var("MSYS2_PATH_TYPE").is_ok() ||
                           env::var("MINGW_PREFIX").is_ok();
            
            println!("cargo:rustc-link-lib=stdc++");
            println!("cargo:rustc-link-lib=gcc");
            println!("cargo:rustc-link-lib=mingw32");
            
            if in_msys2 {
                // MSYS2/MinGW64 environments use UCRT for better compatibility
                println!("cargo:rustc-link-lib=ucrt");
                println!("info: Using UCRT runtime for MSYS2/MinGW64 compatibility");
            } else {
                // Fallback to MSVCRT for older/different MinGW distributions
                println!("cargo:rustc-link-lib=msvcrt");
            }
        }
    } else {
        // MSVC builds
        println!("cargo:rustc-link-lib=msvcrt");
    }

    // Windows system libraries
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=gdiplus");
    println!("cargo:rustc-link-lib=msimg32");
    println!("cargo:rustc-link-lib=comdlg32");
    println!("cargo:rustc-link-lib=winspool");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=shlwapi");
    println!("cargo:rustc-link-lib=comctl32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=rpcrt4");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=version");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=wininet");
    println!("cargo:rustc-link-lib=oleacc");
    println!("cargo:rustc-link-lib=uxtheme");
    println!("cargo:rustc-link-lib=imm32");
}

fn link_linux_libraries() {
    // Core wxWidgets libraries for Linux
    println!("cargo:rustc-link-lib=static=wx_gtk3u_core-3.3");
    println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_adv-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_gl-3.3");
    println!("cargo:rustc-link-lib=static=wx_gtk3u_propgrid-3.3");

    // Conditional feature libraries
    if cfg!(feature = "aui") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_aui-3.3");
    }
    if cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_webview-3.3");
    }
    if cfg!(feature = "xrc") || cfg!(feature = "webview") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_html-3.3");
    }
    if cfg!(feature = "media-ctrl") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_media-3.3");
    }
    if cfg!(feature = "stc") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_stc-3.3");
        println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
        println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
    }
    if cfg!(feature = "xrc") {
        println!("cargo:rustc-link-lib=static=wx_gtk3u_xrc-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3");
    }

    // System libraries
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=xkbcommon");

    // GTK and system libraries via pkg-config
    let lib = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
    for l in lib.libs {
        println!("cargo:rustc-link-lib={}", l);
    }

    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=png");
    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=expat");
    println!("cargo:rustc-link-lib=tiff");
}

fn build_wxdragon_wrapper(
    out_dir: &Path,
    target_os: &str,
    target_env: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    // Get the pre-built wxWidgets library directory
    let artifact_name = match (target_os, target_arch.as_str(), target_env) {
        ("linux", "x86_64", _) => "wxwidgets-linux-x64",
        ("macos", "x86_64", _) => "wxwidgets-macos-x64",
        ("macos", "aarch64", _) => "wxwidgets-macos-arm64",
        ("windows", "x86_64", "msvc") => "wxwidgets-windows-msvc-x64",
        ("windows", "x86_64", "gnu") => "wxwidgets-windows-gnu-x64",
        _ => {
            return Err(format!(
                "Unsupported target platform: {}-{}-{}",
                target_os, target_arch, target_env
            )
            .into())
        }
    };

    let wx_lib_dir = out_dir.join(artifact_name);
    let wrapper_build_dir = out_dir.join("wxdragon_wrapper_build");

    // Skip build if already built (CMake puts libraries in lib/ subdirectory)
    let output_lib = wrapper_build_dir.join("lib").join("libwxdragon.a");
    if output_lib.exists() {
        println!(
            "info: Using cached wxDragon wrapper library at {:?}",
            output_lib
        );

        // Copy to output directory for linking
        let dest = out_dir.join(&artifact_name).join("libwxdragon.a");
        std::fs::create_dir_all(dest.parent().unwrap())?;
        std::fs::copy(&output_lib, &dest)?;
        return Ok(());
    }

    println!("info: Building wxDragon wrapper library...");

    // Create build directory
    std::fs::create_dir_all(&wrapper_build_dir)?;

    // Get absolute path to the cpp source directory
    let cpp_source_dir = env::var("CARGO_MANIFEST_DIR")
        .map(|manifest_dir| Path::new(&manifest_dir).join("cpp"))
        .unwrap_or_else(|_| Path::new("rust/wxdragon-sys/cpp").to_path_buf());

    // Prepare CMake command
    let mut cmake_cmd = std::process::Command::new("cmake");
    cmake_cmd
        .current_dir(&wrapper_build_dir)
        .arg(&cpp_source_dir) // Use absolute path to cpp source directory
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .arg(format!("-DWXWIDGETS_LIB_DIR={}", wx_lib_dir.display()));

    // Pass Cargo feature flags to CMake
    cmake_cmd.arg(format!(
        "-DwxdUSE_AUI={}",
        if cfg!(feature = "aui") { "ON" } else { "OFF" }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_MEDIACTRL={}",
        if cfg!(feature = "media-ctrl") {
            "ON"
        } else {
            "OFF"
        }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_WEBVIEW={}",
        if cfg!(feature = "webview") {
            "ON"
        } else {
            "OFF"
        }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_STC={}",
        if cfg!(feature = "stc") { "ON" } else { "OFF" }
    ));
    cmake_cmd.arg(format!(
        "-DwxdUSE_XRC={}",
        if cfg!(feature = "xrc") { "ON" } else { "OFF" }
    ));

    // Platform-specific CMake configuration
    // Detect host platform for cross-compilation scenarios
    let host_os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };

    // Explicitly set target system for cross-compilation
    if target_os != host_os {
        match target_os {
            "windows" => {
                cmake_cmd.arg("-DCMAKE_SYSTEM_NAME=Windows");
                // Set target architecture
                if target_arch == "x86_64" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86_64");
                }

                // For cross-compilation from Unix to Windows GNU, we need to set up MinGW toolchain
                if target_env == "gnu" && host_os != "windows" {
                    // Try to find MinGW-w64 cross compiler
                    let cross_compiler = format!("{}-w64-mingw32-g++", target_arch);

                    // Check if the cross compiler exists
                    if std::process::Command::new("which")
                        .arg(&cross_compiler)
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false)
                    {
                        cmake_cmd.arg(format!("-DCMAKE_CXX_COMPILER={}", cross_compiler));
                        cmake_cmd.arg(format!(
                            "-DCMAKE_C_COMPILER={}-w64-mingw32-gcc",
                            target_arch
                        ));
                        println!("info: Using MinGW-w64 cross compiler: {}", cross_compiler);
                    } else {
                        println!("cargo:warning=MinGW-w64 cross compiler not found. Cross-compilation to Windows GNU may fail.");
                        println!("cargo:warning=Consider installing mingw-w64 with: brew install mingw-w64");
                    }
                }
            }
            "linux" => {
                cmake_cmd.arg("-DCMAKE_SYSTEM_NAME=Linux");
                if target_arch == "x86_64" {
                    cmake_cmd.arg("-DCMAKE_SYSTEM_PROCESSOR=x86_64");
                }
            }
            _ => {}
        }
    }

    match target_os {
        "windows" => {
            if target_env == "msvc" {
                cmake_cmd.arg("-G").arg("Ninja");
            } else {
                // GNU/MinGW64 - choose generator based on environment
                if host_os == "windows" {
                    // Check if we're in MSYS2 environment (like GitHub Actions)
                    // MSYS2 usually sets these environment variables
                    let in_msys2 = env::var("MSYSTEM").is_ok() || 
                                   env::var("MSYS2_PATH_TYPE").is_ok() ||
                                   env::var("MINGW_PREFIX").is_ok();
                    
                    if in_msys2 {
                        // In MSYS2, use Unix Makefiles for better compatibility
                        cmake_cmd.arg("-G").arg("Unix Makefiles");
                        
                        // Explicitly set compilers for MSYS2/MinGW64
                        cmake_cmd.arg("-DCMAKE_C_COMPILER=gcc");
                        cmake_cmd.arg("-DCMAKE_CXX_COMPILER=g++");
                        cmake_cmd.arg("-DCMAKE_AR=ar");
                        cmake_cmd.arg("-DCMAKE_MAKE_PROGRAM=make");
                        
                        println!("info: Detected MSYS2 environment, using Unix Makefiles generator");
                    } else {
                        // Native Windows MinGW (not MSYS2)
                        cmake_cmd.arg("-G").arg("MSYS Makefiles");
                    }
                } else {
                    // Cross-compiling from Unix-like system (macOS/Linux)
                    cmake_cmd.arg("-G").arg("Unix Makefiles");
                }
            }
        }
        "macos" => {
            cmake_cmd.arg("-DCMAKE_OSX_DEPLOYMENT_TARGET=10.13");
            if target_arch == "aarch64" {
                cmake_cmd.arg("-DCMAKE_OSX_ARCHITECTURES=arm64");
            } else {
                cmake_cmd.arg("-DCMAKE_OSX_ARCHITECTURES=x86_64");
            }
        }
        _ => {} // Linux uses default
    }

    // Configure
    let output = cmake_cmd.output()?;
    if !output.status.success() {
        return Err(format!(
            "CMake configure failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Build
    let mut build_cmd = std::process::Command::new("cmake");
    build_cmd
        .current_dir(&wrapper_build_dir)
        .arg("--build")
        .arg(".")
        .arg("--config")
        .arg("Release");

    let output = build_cmd.output()?;
    if !output.status.success() {
        return Err(format!(
            "CMake build failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    // Copy the built library to the wxWidgets library directory for linking
    let dest = wx_lib_dir.join("libwxdragon.a");
    if !output_lib.exists() {
        return Err("wxDragon wrapper library was not built successfully".into());
    }

    std::fs::copy(&output_lib, &dest)?;
    println!("info: Successfully built wxDragon wrapper library");

    Ok(())
}

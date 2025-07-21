use std::path::PathBuf;

fn main() {
    println!("Building wxdragon-sys...");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target = std::env::var("TARGET").unwrap();

    println!("info: Target OS: {target_os}, Target Env: {target_env}, Target: {target}");

    let wxwidgets_version = "3.3.0";
    let wxwidgets_dir = std::env::current_dir()
        .expect("Failed to get current dir")
        .join(format!("wxWidgets-{wxwidgets_version}"))
        .to_str()
        .expect("Failed to convert wxWidgets directory path to string")
        .to_string();

    if !std::path::Path::new(&wxwidgets_dir).exists() {
        let local_wxwidgets_zip = format!("wxWidgets-{wxwidgets_version}.zip");
        if !std::path::Path::new(&local_wxwidgets_zip).exists() {
            download_wxwidgets_source(wxwidgets_version, &local_wxwidgets_zip).unwrap();
        }
        extract_wxwidgets_source(&local_wxwidgets_zip, &wxwidgets_dir).unwrap();
    } else {
        println!("wxWidgets source directory {wxwidgets_dir} already exists, skipping download and extraction.");
    }

    // --- 1. Generate FFI Bindings ---
    println!("info: Generating FFI bindings...");

    let mut bindings_builder = bindgen::Builder::default()
        .header("cpp/include/wxdragon.h")
        .clang_arg(format!("-I{wxwidgets_dir}/include"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Add feature flags for conditional compilation
    if cfg!(feature = "aui") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_AUI=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_AUI=0");
    }

    if cfg!(feature = "media-ctrl") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_MEDIACTRL=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_MEDIACTRL=0");
    }

    if cfg!(feature = "webview") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_WEBVIEW=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_WEBVIEW=0");
    }

    if cfg!(feature = "stc") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_STC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_STC=0");
    }

    if cfg!(feature = "xrc") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_XRC=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_XRC=0");
    }

    if cfg!(feature = "richtext") {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_RICHTEXT=1");
    } else {
        bindings_builder = bindings_builder.clang_arg("-DwxUSE_RICHTEXT=0");
    }

    bindings_builder = bindings_builder.clang_arg(format!("--target={target}"));

    // Skip library setup for docs.rs and rust-analyzer
    use std::env::var;
    if var("DOCS_RS").is_ok() || std::env::var("RUST_ANALYZER") == Ok("true".to_string()) {
        let bindings = bindings_builder
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(out_dir.join("bindings.rs"))
            .expect("Couldn't write bindings!");

        println!("info: Successfully generated FFI bindings");
        return;
    }

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("info: Successfully generated FFI bindings");

    // --- 4. Build wxDragon Wrapper ---
    let wxwidgets_dir = PathBuf::from(wxwidgets_dir);
    build_wxdragon_wrapper(&target, &wxwidgets_dir, &target_os, &target_env)
        .expect("Failed to build wxDragon wrapper library");

    // --- 5. Setup Linking ---
    // setup_linking(wx_version, &target_os, &target_env, &out_dir);
}

fn download_wxwidgets_source(version: &str, output: &str) -> std::io::Result<()> {
    let url = format!(
        "https://github.com/wxWidgets/wxWidgets/releases/download/v{version}/wxWidgets-{version}.zip",
    );

    // if the zip file does not exist, download it
    if !std::path::Path::new(&output).exists() {
        println!("Downloading {url}...");
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .map_err(|e| std::io::Error::other(format!("HTTP client creation failed: {e}")))?;
        let response = client
            .get(&url)
            .send()
            .map_err(|e| std::io::Error::other(format!("HTTP request failed: {e}")))?;
        if !response.status().is_success() {
            return Err(std::io::Error::other(format!(
                "Failed to download file: HTTP status {}",
                response.status()
            )));
        }
        let mut file = std::fs::File::create(output)
            .map_err(|e| std::io::Error::other(format!("Failed to create output file: {e}")))?;
        let content = response
            .bytes()
            .map_err(|e| std::io::Error::other(format!("Failed to read response bytes: {e}")))?;
        std::io::copy(&mut content.as_ref(), &mut file)
            .map_err(|e| std::io::Error::other(format!("Failed to write to output file: {e}")))?;
        println!("Downloaded {output} successfully.");
    } else {
        println!("File {output} already exists, skipping download.");
    }
    Ok(())
}

fn extract_wxwidgets_source<P: AsRef<std::path::Path>>(
    zip_path: &str,
    output_dir: P,
) -> std::io::Result<()> {
    let output_dir_str = output_dir.as_ref().to_str().unwrap();
    if output_dir.as_ref().exists() {
        println!("Output directory {output_dir_str} already exists, skipping extraction.");
        return Ok(());
    }

    if !std::path::Path::new(zip_path).exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Zip file {zip_path} does not exist."),
        ));
    }

    let file = std::fs::File::open(zip_path)
        .map_err(|e| std::io::Error::other(format!("Failed to open zip file: {e}")))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| std::io::Error::other(format!("Failed to read zip archive: {e}")))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| std::io::Error::other(format!("Failed to access file in zip: {e}")))?;
        let outpath = match file.enclosed_name() {
            Some(path) => output_dir.as_ref().join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            // Directory
            std::fs::create_dir_all(&outpath)
                .map_err(|e| std::io::Error::other(format!("Failed to create directory: {e}")))?;
        } else {
            // File
            if let Some(parent) = outpath.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    std::io::Error::other(format!("Failed to create parent directory: {e}"))
                })?;
            }
            let mut outfile = std::fs::File::create(&outpath)
                .map_err(|e| std::io::Error::other(format!("Failed to create output file: {e}")))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| {
                std::io::Error::other(format!("Failed to write to output file: {e}"))
            })?;
        }
    }
    println!("Extracted wxWidgets source to {output_dir_str}");
    Ok(())
}

fn build_wxdragon_wrapper(
    target: &str,
    wxwidgets_source_path: &PathBuf,
    target_os: &str,
    target_env: &str,
) -> std::io::Result<()> {
    // --- 3. Configure and Build libwxdragon (and wxWidgets) using CMake ---
    let libwxdragon_cmake_source_dir = PathBuf::from("cpp");

    let mut cmake_config = cmake::Config::new(libwxdragon_cmake_source_dir);
    cmake_config.define("WXWIDGETS_SOURCE_PATH", wxwidgets_source_path);

    // Disable WebP support since we'll use the image crate for image decoding
    cmake_config.define("wxUSE_LIBWEBP", "OFF");

    cmake_config.define(
        "wxUSE_AUI",
        if cfg!(feature = "aui") { "ON" } else { "OFF" },
    );

    cmake_config.define(
        "wxUSE_MEDIACTRL",
        if cfg!(feature = "media-ctrl") {
            "ON"
        } else {
            "OFF"
        },
    );

    cmake_config.define(
        "wxUSE_WEBVIEW",
        if cfg!(feature = "webview") {
            "ON"
        } else {
            "OFF"
        },
    );

    cmake_config.define(
        "wxUSE_STC",
        if cfg!(feature = "stc") { "ON" } else { "OFF" },
    );

    cmake_config.define(
        "wxUSE_XRC",
        if cfg!(feature = "xrc") { "ON" } else { "OFF" },
    );

    // wxUSE_RICHTEXT
    cmake_config.define(
        "wxUSE_RICHTEXT",
        if cfg!(feature = "richtext") {
            "ON"
        } else {
            "OFF"
        },
    );

    let mut is_debug = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string()) == "debug";
    if target_os == "windows" {
        is_debug = false;
        if target_env == "gnu" {
            // Potentially set MinGW toolchain for CMake if not automatically detected
        } else {
            cmake_config.generator("Ninja");
        }

        if target == "i686-pc-windows-msvc" {
            cmake_config
                .generator("Visual Studio 17 2022")
                .define("CMAKE_GENERATOR_PLATFORM", "Win32")
                .cxxflag("/EHsc");
        }
    }

    // Set CMake build type based on Rust profile
    cmake_config.define(
        "CMAKE_BUILD_TYPE",
        if is_debug { "Debug" } else { "Release" },
    );

    let dst = cmake_config.build();
    let build_dir = dst.join("build");
    let lib_search_path = build_dir.join("lib").display().to_string();
    let wxwidgets_build_dir = wxwidgets_source_path.join("wxwidgets_build");

    println!("info: CMake build completed. Build directory: {build_dir:?}");
    println!("info: libwxdragon should be in: {lib_search_path:?}");
    println!("info: wxWidgets build subdirectory: {wxwidgets_build_dir:?}");

    // --- 4. Linker Instructions ---
    println!("cargo:rustc-link-search=native={lib_search_path}");

    let v = wxwidgets_build_dir.join("lib").display().to_string();
    println!("cargo:rustc-link-search=native={v}");

    // For Windows, wxWidgets libs might be in a subdirectory like gcc_x64_lib for MinGW
    if target_os == "windows" {
        if target_env == "gnu" {
            let v = wxwidgets_build_dir
                .join("lib/gcc_x64_lib")
                .display()
                .to_string();
            println!("cargo:rustc-link-search=native={v}");

            // --- Dynamically find MinGW GCC library paths ---
            let gcc_path = "x86_64-w64-mingw32-gcc"; // Assume it's in PATH

            // Find the path containing libgcc.a
            let output_libgcc = std::process::Command::new(gcc_path)
                .arg("-print-libgcc-file-name")
                .output()
                .expect("Failed to execute x86_64-w64-mingw32-gcc -print-libgcc-file-name");

            if output_libgcc.status.success() {
                let libgcc_path_str = String::from_utf8_lossy(&output_libgcc.stdout)
                    .trim()
                    .to_string();
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
                                    // Construct the expected path for libstdc++.a based on `find` result structure
                                    let libstdcpp_dir = base_lib_dir
                                        .parent()
                                        .unwrap()
                                        .join("x86_64-w64-mingw32/lib"); // ../../x86_64-w64-mingw32/lib
                                    let v = libstdcpp_dir.display();
                                    if libstdcpp_dir.exists() && libstdcpp_dir != libgcc_dir {
                                        println!("cargo:rustc-link-search=native={v}");
                                        println!(
                                            "info: Add GCC lib search path(for libstdc++):{v}"
                                        );
                                    } else {
                                        println!("info: Could not find or verify expected libstdc++ path relative to libgcc path: {v}");
                                    }
                                }
                            }
                        }
                    } else {
                        println!("cargo:warning=Could not get parent directory from libgcc path: {libgcc_path_str}");
                    }
                } else {
                    println!(
                        "cargo:warning=Command -print-libgcc-file-name returned empty output."
                    );
                }
            } else {
                let stderr = String::from_utf8_lossy(&output_libgcc.stderr);
                println!(
                    "cargo:warning=Failed to run '{gcc_path} -print-libgcc-file-name': {stderr}"
                );
                println!("cargo:warning=Static linking for stdc++/gcc might fail. Falling back to hoping they are in default paths.");
            }
            // --- End dynamic path finding ---
        } else {
            let lib_dir = if target == "i686-pc-windows-msvc" {
                "lib/vc_lib"
            } else {
                "lib/vc_x64_lib"
            };
            println!(
                "cargo:rustc-link-search=native={}",
                wxwidgets_build_dir.join(lib_dir).display()
            );
        }
    }

    println!("cargo:rustc-link-lib=static=wxdragon");

    if target_os == "macos" {
        // macOS linking flags (assuming release build for wxWidgets library names here)
        // If macOS also has d suffix for debug, this section would need similar conditional logic
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_core-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_adv-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_gl-3.3");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_propgrid-3.3");

        // Conditional features for macOS
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
        if cfg!(feature = "richtext") {
            println!("cargo:rustc-link-lib=static=wx_osx_cocoau_richtext-3.3");
        }

        println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
        println!("cargo:rustc-link-lib=static=wxpng-3.3");
        println!("cargo:rustc-link-lib=static=wxtiff-3.3");
        println!("cargo:rustc-link-lib=static=wxregexu-3.3");
        println!("cargo:rustc-link-lib=expat");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=c++");

        // Conditional STC support libraries for macOS
        if cfg!(feature = "stc") {
            println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
            println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
        }

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
    } else if target_os == "windows" {
        // Detect cross-compilation from macOS to Windows
        let host_os = std::env::consts::OS;
        let is_macos_to_windows_gnu =
            host_os == "macos" && target_os == "windows" && target_env == "gnu";

        if is_macos_to_windows_gnu {
            // Cross-compilation from macOS: libraries have -Windows suffix
            println!("cargo:rustc-link-lib=static=wx_mswu_core-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_adv-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_baseu-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_gl-3.3-Windows");
            println!("cargo:rustc-link-lib=static=wx_mswu_propgrid-3.3-Windows");

            // Conditional features for cross-compilation
            if cfg!(feature = "aui") {
                println!("cargo:rustc-link-lib=static=wx_mswu_aui-3.3-Windows");
            }
            if cfg!(feature = "media-ctrl") {
                println!("cargo:rustc-link-lib=static=wx_mswu_media-3.3-Windows");
            }
            if cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wx_mswu_webview-3.3-Windows");
            }
            if cfg!(feature = "xrc") || cfg!(feature = "webview") {
                println!("cargo:rustc-link-lib=static=wx_mswu_html-3.3-Windows");
            }
            if cfg!(feature = "stc") {
                println!("cargo:rustc-link-lib=static=wx_mswu_stc-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wxscintilla-3.3");
                println!("cargo:rustc-link-lib=static=wxlexilla-3.3");
            }
            if cfg!(feature = "xrc") {
                println!("cargo:rustc-link-lib=static=wx_mswu_xrc-3.3-Windows");
                println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.3-Windows");
            }
            if cfg!(feature = "richtext") {
                println!("cargo:rustc-link-lib=static=wx_mswu_richtext-3.3-Windows");
            }

            println!("cargo:rustc-link-lib=static=wxpng-3.3");
            println!("cargo:rustc-link-lib=static=wxtiff-3.3");
            println!("cargo:rustc-link-lib=static=wxjpeg-3.3");
            println!("cargo:rustc-link-lib=static=wxregexu-3.3");
            println!("cargo:rustc-link-lib=static=wxzlib-3.3");
            println!("cargo:rustc-link-lib=static=wxexpat-3.3");

            println!("info: Using static linking for cross-compilation from macOS to Windows GNU");
            // Static linking for cross-compilation to avoid runtime dependencies
            println!("cargo:rustc-link-lib=static=stdc++");
            println!("cargo:rustc-link-lib=static=gcc");
            println!("cargo:rustc-link-lib=static=gcc_eh");
            println!("cargo:rustc-link-lib=static=pthread");
            // Add linker arguments for fully static C++ runtime
            println!("cargo:rustc-link-arg=-static-libgcc");
            println!("cargo:rustc-link-arg=-static-libstdc++");
        } else {
            if is_debug {
                // Native Windows (both MSVC and GNU): debug libraries
                println!("cargo:rustc-link-lib=static=wxmsw33ud_adv");
                println!("cargo:rustc-link-lib=static=wxmsw33ud_core");
                println!("cargo:rustc-link-lib=static=wxmsw33ud_gl");
                println!("cargo:rustc-link-lib=static=wxmsw33ud_propgrid");

                // Conditional features for Windows debug
                if cfg!(feature = "aui") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_aui");
                }
                if cfg!(feature = "media-ctrl") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_media");
                }
                if cfg!(feature = "webview") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_webview");
                }
                if cfg!(feature = "xrc") || cfg!(feature = "webview") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_html");
                }
                if cfg!(feature = "stc") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_stc");
                    println!("cargo:rustc-link-lib=static=wxscintillad");
                    println!("cargo:rustc-link-lib=static=wxlexillad");
                }
                if cfg!(feature = "xrc") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_xrc");
                    println!("cargo:rustc-link-lib=static=wxbase33ud_xml");
                }
                if cfg!(feature = "richtext") {
                    println!("cargo:rustc-link-lib=static=wxmsw33ud_richtext");
                }

                println!("cargo:rustc-link-lib=static=wxbase33ud");
                println!("cargo:rustc-link-lib=static=wxpngd");
                println!("cargo:rustc-link-lib=static=wxtiffd");
                println!("cargo:rustc-link-lib=static=wxjpegd");
                println!("cargo:rustc-link-lib=static=wxregexud");
                println!("cargo:rustc-link-lib=static=wxzlibd");
                println!("cargo:rustc-link-lib=static=wxexpatd");
            } else {
                // Native Windows (both MSVC and GNU): use same library names
                println!("cargo:rustc-link-lib=static=wxmsw33u_adv");
                println!("cargo:rustc-link-lib=static=wxmsw33u_core");
                println!("cargo:rustc-link-lib=static=wxmsw33u_gl");
                println!("cargo:rustc-link-lib=static=wxmsw33u_propgrid");

                // Conditional features for native Windows
                if cfg!(feature = "aui") {
                    println!("cargo:rustc-link-lib=static=wxmsw33u_aui");
                }
                if cfg!(feature = "media-ctrl") {
                    println!("cargo:rustc-link-lib=static=wxmsw33u_media");
                }
                if cfg!(feature = "webview") {
                    println!("cargo:rustc-link-lib=static=wxmsw33u_webview");
                }
                if cfg!(feature = "xrc") || cfg!(feature = "webview") {
                    println!("cargo:rustc-link-lib=static=wxmsw33u_html");
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
                if cfg!(feature = "richtext") {
                    println!("cargo:rustc-link-lib=static=wxmsw33u_richtext");
                }

                println!("cargo:rustc-link-lib=static=wxbase33u");
                println!("cargo:rustc-link-lib=static=wxtiff");
                println!("cargo:rustc-link-lib=static=wxjpeg");
                println!("cargo:rustc-link-lib=static=wxpng");
                println!("cargo:rustc-link-lib=static=wxregexu");
                println!("cargo:rustc-link-lib=static=wxzlib");
                println!("cargo:rustc-link-lib=static=wxexpat");
            }

            if target_env == "gnu" {
                println!("cargo:rustc-link-lib=stdc++");
            }
        }

        // System libraries (same for debug and release)
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        println!("cargo:rustc-link-lib=gdiplus"); // Add GDI+ library for graphics support
        println!("cargo:rustc-link-lib=msimg32"); // Add for AlphaBlend and GradientFill functions
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
        println!("cargo:rustc-link-lib=imm32"); // Add IME library for Scintilla support
    } else {
        // For Linux and other Unix-like systems
        println!("cargo:rustc-link-lib=xkbcommon");
        let lib = pkg_config::Config::new().probe("gtk+-3.0").unwrap();
        for _lib in lib.libs {
            println!("cargo:rustc-link-lib={_lib}");
        }
        println!("cargo:rustc-link-lib=X11");
        println!("cargo:rustc-link-lib=png");
        println!("cargo:rustc-link-lib=jpeg");
        println!("cargo:rustc-link-lib=expat");
        println!("cargo:rustc-link-lib=tiff");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_propgrid-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_gl-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_adv-3.3");
        println!("cargo:rustc-link-lib=static=wx_gtk3u_core-3.3");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.3");
        println!("cargo:rustc-link-lib=stdc++");

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
        if cfg!(feature = "richtext") {
            println!("cargo:rustc-link-lib=static=wx_gtk3u_richtext-3.3");
        }
    }

    Ok(())
}

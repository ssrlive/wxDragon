use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp/include/wxdragon.h");
    println!("cargo:rerun-if-changed=cpp/include/wxd_types.h");
    println!("cargo:rerun-if-changed=cpp/include/core/wxd_app.h");
    println!("cargo:rerun-if-changed=cpp/include/core/wxd_window_base.h");
    println!("cargo:rerun-if-changed=cpp/include/core/wxd_xrc.h");
    println!("cargo:rerun-if-changed=cpp/include/dialogs/wxd_dialogs.h");
    println!("cargo:rerun-if-changed=cpp/include/events/wxd_event_api.h");
    println!("cargo:rerun-if-changed=cpp/include/sizers/wxd_sizers.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_choices.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_containers.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_controls.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_button.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_bitmapbutton.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_misc_widgets.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_pickers.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_listctrl.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_treectrl.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_dataview.h");
    println!("cargo:rerun-if-changed=cpp/include/widgets/wxd_notebook.h");
    println!("cargo:rerun-if-changed=cpp/src");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let is_debug = profile == "debug";

    let wx_version = "3.2.8";
    let wx_tarball_name = format!("wxWidgets-{}.tar.bz2", wx_version);
    let wx_download_url = format!(
        "https://github.com/wxWidgets/wxWidgets/releases/download/v{}/{}",
        wx_version, wx_tarball_name
    );

    let tarball_dest_path = out_dir.join(&wx_tarball_name);
    let wx_extracted_source_path = out_dir.join(format!("wxWidgets-{}", wx_version));

    // --- 1. Download and Extract wxWidgets Source ---
    if !wx_extracted_source_path.exists() {
        if !tarball_dest_path.exists() {
            println!(
                "info: Downloading {} from {}",
                wx_tarball_name, wx_download_url
            );
            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(300))
                .build()
                .expect("Failed to build reqwest client");

            let mut resp = client
                .get(&wx_download_url)
                .send()
                .expect(&format!("Failed to download {}", wx_download_url));

            if !resp.status().is_success() {
                panic!(
                    "Failed to download {}: HTTP {}",
                    wx_download_url,
                    resp.status()
                );
            }

            let mut out_file = File::create(&tarball_dest_path).expect(&format!(
                "Failed to create destination file {:?}",
                tarball_dest_path
            ));
            std::io::copy(&mut resp, &mut out_file).expect(&format!(
                "Failed to write downloaded content to {:?}",
                tarball_dest_path
            ));
        } else {
            println!(
                "info: Using cached wxWidgets tarball: {:?}",
                tarball_dest_path
            );
        }

        println!(
            "info: Extracting {} to output directory root ({:?})",
            wx_tarball_name, out_dir
        );

        let tarball_file = File::open(&tarball_dest_path)
            .expect(&format!("Failed to open tarball {:?}", tarball_dest_path));
        let bz_decoder = bzip2::read::BzDecoder::new(tarball_file);
        let mut archive = tar::Archive::new(bz_decoder);

        archive.unpack(&out_dir).expect(&format!(
            "Failed to extract {} to {:?}",
            wx_tarball_name, out_dir
        ));

        if !wx_extracted_source_path.exists() {
            panic!(
                "Extraction did not result in expected directory: {:?}. Check tarball structure.",
                wx_extracted_source_path
            );
        }
        println!(
            "info: Successfully extracted wxWidgets to {:?}",
            wx_extracted_source_path
        );
    } else {
        println!(
            "info: Using existing extracted wxWidgets source at {:?}",
            wx_extracted_source_path
        );
    }

    // --- 2. Configure and Build libwxdragon (and wxWidgets) using CMake ---
    let libwxdragon_cmake_source_dir = PathBuf::from("cpp");
    let mut cmake_config = cmake::Config::new(libwxdragon_cmake_source_dir);
    cmake_config.define("WXWIDGETS_SOURCE_DIR", &wx_extracted_source_path);

    // Set CMake build type based on Rust profile
    if is_debug {
        cmake_config.define("CMAKE_BUILD_TYPE", "Debug");
    } else {
        cmake_config.define("CMAKE_BUILD_TYPE", "Release");
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_os == "windows" && target_env == "gnu" {
        // Potentially set MinGW toolchain for CMake if not automatically detected
    }

    let dst = cmake_config.build();
    let build_dir = dst.join("build");
    let lib_search_path = build_dir.join("lib");
    let wxwidgets_build_dir = build_dir.join("wxwidgets_build");

    println!(
        "info: CMake build completed. Build directory: {:?}",
        build_dir
    );
    println!("info: libwxdragon should be in: {:?}", lib_search_path);
    println!(
        "info: wxWidgets build subdirectory: {:?}",
        wxwidgets_build_dir
    );

    // --- 3. Linker Instructions ---
    println!(
        "cargo:rustc-link-search=native={}",
        lib_search_path.display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        wxwidgets_build_dir.join("lib").display()
    );
    // For Windows, wxWidgets libs might be in a subdirectory like gcc_x64_lib for MinGW
    if target_os == "windows" && target_env == "gnu" {
        println!(
            "cargo:rustc-link-search=native={}",
            wxwidgets_build_dir.join("lib/gcc_x64_lib").display()
        );

        // --- Dynamically find MinGW GCC library paths ---
        let gcc_path = "x86_64-w64-mingw32-gcc"; // Assume it's in PATH

        // Find the path containing libgcc.a
        let output_libgcc = Command::new(gcc_path)
            .arg("-print-libgcc-file-name")
            .output()
            .expect("Failed to execute x86_64-w64-mingw32-gcc -print-libgcc-file-name");

        if output_libgcc.status.success() {
            let libgcc_path_str = String::from_utf8_lossy(&output_libgcc.stdout)
                .trim()
                .to_string();
            if !libgcc_path_str.is_empty() {
                let libgcc_path = Path::new(&libgcc_path_str);
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
                println!("cargo:warning=Command -print-libgcc-file-name returned empty output.");
            }
        } else {
            let stderr = String::from_utf8_lossy(&output_libgcc.stderr);
            println!(
                "cargo:warning=Failed to run '{} -print-libgcc-file-name': {}",
                gcc_path, stderr
            );
            println!("cargo:warning=Static linking for stdc++/gcc might fail. Falling back to hoping they are in default paths.");
        }
        // --- End dynamic path finding ---

        // REMOVED: Old hardcoded path
        // println!("cargo:rustc-link-search=native=/opt/homebrew/Cellar/mingw-w64/12.0.0_3/toolchain-x86_64/x86_64-w64-mingw32/lib");
    }

    println!("cargo:rustc-link-lib=static=wxdragon");

    if target_os == "macos" {
        // macOS linking flags (assuming release build for wxWidgets library names here)
        // If macOS also has d suffix for debug, this section would need similar conditional logic
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_core-3.2");
        println!("cargo:rustc-link-lib=static=wx_baseu-3.2");
        println!("cargo:rustc-link-lib=static=wx_baseu_xml-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_adv-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_aui-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_gl-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_html-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_media-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_propgrid-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_stc-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_webview-3.2");
        println!("cargo:rustc-link-lib=static=wx_osx_cocoau_xrc-3.2");
        println!("cargo:rustc-link-lib=static=wxjpeg-3.2");
        println!("cargo:rustc-link-lib=static=wxpng-3.2");
        println!("cargo:rustc-link-lib=static=wxtiff-3.2");
        println!("cargo:rustc-link-lib=static=wxregexu-3.2");
        println!("cargo:rustc-link-lib=static=wxscintilla-3.2");
        println!("cargo:rustc-link-lib=expat");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=iconv");
        println!("cargo:rustc-link-lib=c++");
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
        println!("cargo:rustc-link-lib=framework=AVFoundation");
        println!("cargo:rustc-link-lib=framework=AVKit");
        println!("cargo:rustc-link-lib=framework=CoreMedia");
    } else if target_os == "windows" && target_env == "gnu" {
        if is_debug {
            println!("info: Using DEBUG linking flags for Windows (GNU).");
            // wxWidgets debug libraries from user's ll output
            println!("cargo:rustc-link-lib=static=wxmsw32ud_aui");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_adv");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_core");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_gl");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_html");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_media");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_propgrid");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_stc");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_webview");
            println!("cargo:rustc-link-lib=static=wxmsw32ud_xrc");
            println!("cargo:rustc-link-lib=static=wxbase32ud_xml");
            println!("cargo:rustc-link-lib=static=wxbase32ud");
            println!("cargo:rustc-link-lib=static=wxpngd");
            println!("cargo:rustc-link-lib=static=wxtiffd");
            println!("cargo:rustc-link-lib=static=wxjpegd");
            println!("cargo:rustc-link-lib=static=wxregexud");
            println!("cargo:rustc-link-lib=static=wxzlibd");
            println!("cargo:rustc-link-lib=static=wxscintillad");
            println!("cargo:rustc-link-lib=static=wxexpatd");
        } else {
            println!("info: Using RELEASE linking flags for Windows (GNU) based on user-provided ll output.");
            // wxWidgets release libraries from user-provided ll output
            println!("cargo:rustc-link-lib=static=wxmsw32u_aui");
            println!("cargo:rustc-link-lib=static=wxmsw32u_adv");
            println!("cargo:rustc-link-lib=static=wxmsw32u_core");
            println!("cargo:rustc-link-lib=static=wxmsw32u_gl");
            println!("cargo:rustc-link-lib=static=wxmsw32u_html");
            println!("cargo:rustc-link-lib=static=wxmsw32u_media");
            println!("cargo:rustc-link-lib=static=wxmsw32u_propgrid");
            println!("cargo:rustc-link-lib=static=wxmsw32u_stc");
            println!("cargo:rustc-link-lib=static=wxmsw32u_webview");
            println!("cargo:rustc-link-lib=static=wxmsw32u_xrc");
            println!("cargo:rustc-link-lib=static=wxbase32u_xml");
            println!("cargo:rustc-link-lib=static=wxbase32u");
            println!("cargo:rustc-link-lib=static=wxtiff");
            println!("cargo:rustc-link-lib=static=wxjpeg");
            println!("cargo:rustc-link-lib=static=wxpng");
            println!("cargo:rustc-link-lib=static=wxregexu");
            println!("cargo:rustc-link-lib=static=wxzlib");
            println!("cargo:rustc-link-lib=static=wxscintilla");
            println!("cargo:rustc-link-lib=static=wxexpat");
        }

        // System libraries (same for debug and release)
        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
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
        println!("cargo:rustc-link-lib=static=stdc++");

        // Add flags for static linking of libstdc++ and libgcc
        println!("cargo:rustc-link-arg=-static-libstdc++");
        println!("cargo:rustc-link-arg=-static-libgcc");
    } else {
        println!("info: Manual linking flags are currently only implemented for macOS and Windows (GNU). Build may fail on other platforms.");
    }

    // --- 4. Bindgen Include Path Setup ---
    println!("info: Setting up include paths for bindgen...");
    let wx_source_include_dir = wx_extracted_source_path.join("include");

    let mut bindings_builder = bindgen::Builder::default()
        .header("cpp/include/wxdragon.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}", wx_source_include_dir.display()));

    if target_os == "macos" {
        // macOS bindgen args (assuming release for setup.h path for now)
        let wx_setup_h_include_path =
            wxwidgets_build_dir.join("lib/wx/include/osx_cocoa-unicode-static-3.2");
        bindings_builder = bindings_builder
            .clang_arg(format!("-I{}", wx_setup_h_include_path.display()))
            .clang_arg("-D__WXOSX_COCOA__")
            .clang_arg("-D__WXMAC__")
            .clang_arg("-D__WXOSX__")
            .clang_arg("-D_FILE_OFFSET_BITS=64");
        if !is_debug {
            // NDEBUG for release, _DEBUG for debug is common for wx
            bindings_builder = bindings_builder.clang_arg("-DNDEBUG");
        } else {
            bindings_builder = bindings_builder
                .clang_arg("-D_DEBUG")
                .clang_arg("-DwxDEBUG_LEVEL=1");
        }
    } else if target_os == "windows" && target_env == "gnu" {
        let wx_setup_h_parent_dir_segment = if is_debug {
            "lib/gcc_x64_lib/mswud" // Path from user's tree output, contains wx/setup.h
        } else {
            "lib/gcc_x64_lib/mswu" // Path from user's tree output, contains wx/setup.h
        };
        let wx_setup_h_include_path = wxwidgets_build_dir.join(wx_setup_h_parent_dir_segment);

        bindings_builder = bindings_builder
            .clang_arg(format!("-I{}", wx_setup_h_include_path.display()))
            .clang_arg("-D__WXMSW__")
            .clang_arg("-D_FILE_OFFSET_BITS=64")
            .clang_arg("-DwxUSE_UNICODE=1");

        if is_debug {
            bindings_builder = bindings_builder
                .clang_arg("-D_DEBUG")
                .clang_arg("-DwxDEBUG_LEVEL=1");
        } else {
            bindings_builder = bindings_builder.clang_arg("-DNDEBUG");
        }
    } else {
        println!("info: Manual bindgen Clang args are currently only implemented for macOS and Windows (GNU). Bindgen may use incomplete include paths on other platforms.");
    }

    let target = env::var("TARGET").unwrap();
    bindings_builder = bindings_builder.clang_arg(format!("--target={}", target));

    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "info: Successfully generated FFI bindings to {:?}",
        out_dir.join("bindings.rs")
    );
}

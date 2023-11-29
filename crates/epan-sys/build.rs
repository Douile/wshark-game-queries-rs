#[cfg(feature = "bindgen")]
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // If we are in docs.rs, there is no need to actually link.
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    // By default, we will just use a pre-generated bindings.rs file. If this feature is turned
    // on, we'll re-generate the bindings at build time.
    #[cfg(feature = "bindgen")]
    generate_bindings();

    link_wireshark();
}

fn link_wireshark() {
    if pkg_config::probe_library("wireshark").is_ok() {
        // pkg-config will handle everything for us
        return;
    }

    println!("cargo:rustc-link-lib=dylib=wireshark");

    if let Ok(libws_dir) = env::var("WIRESHARK_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", libws_dir);
    } else {
        // We'll have to pull wireshark in and build it...
        println!("cargo:warning=libwireshark was not found, will be built from source");

        clone_wireshark_or_die();
        let dst = build_wireshark();

        let mut dylib_dir = dst;
        dylib_dir.push("lib");

        println!(
            "cargo:rustc-link-search=native={}",
            dylib_dir.to_string_lossy()
        );
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(false);

    match pkg_config::probe_library("wireshark") {
        Ok(libws) => {
            for path in libws.include_paths {
                builder = builder.clang_arg(format!("-I{}", path.to_string_lossy()));
            }
        }
        Err(_) => {
            let glib = pkg_config::Config::new()
                .probe("glib-2.0")
                .expect("glib-2.0 must be installed");

            for path in glib.include_paths {
                builder = builder.clang_arg(format!("-I{}", path.to_string_lossy()));
            }

            clone_wireshark_or_die();
            let dst = build_wireshark();

            let mut ws_headers_path = dst;
            ws_headers_path.push("include");
            ws_headers_path.push("wireshark");

            builder = builder.clang_arg(format!("-I{}", ws_headers_path.to_string_lossy()));
        }
    }

    let bindings = builder
        .generate()
        .expect("should be able to generate bindings from wrapper.h");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("generated bindings should be written to file");
}

fn clone_wireshark_or_die() {
    if let Ok(dir) = std::fs::metadata("wireshark") {
        assert!(dir.is_dir(), "Wireshark directory should be dir");
    } else {
        let output = Command::new("git")
            .args([
                "clone",
                "https://gitlab.com/wireshark/wireshark.git",
                "--recurse-submodules",
            ])
            .output()
            .expect("wireshark should be obtained as a git submodule");
        assert!(output.status.success(), "Failed to clone wireshark");
    }

    // Checkout version 4.2
    let output = Command::new("git")
        .args(["--work-tree=wireshark", "checkout", "v4.2.0"])
        .output()
        .expect("wireshark should have version 4.2");
    assert!(output.status.success(), "Failed to checkout wireshark 4.2");
}

fn build_wireshark() -> PathBuf {
    cmake::Config::new("wireshark")
        .define("BUILD_androiddump", "OFF")
        .define("BUILD_capinfos", "OFF")
        .define("BUILD_captype", "OFF")
        .define("BUILD_ciscodump", "OFF")
        .define("BUILD_corbaidl2wrs", "OFF")
        .define("BUILD_dcerpcidl2wrs", "OFF")
        .define("BUILD_dftest", "OFF")
        .define("BUILD_dpauxmon", "OFF")
        .define("BUILD_dumpcap", "OFF")
        .define("BUILD_editcap", "OFF")
        .define("BUILD_etwdump", "OFF")
        .define("BUILD_logray", "OFF")
        .define("BUILD_mergecap", "OFF")
        .define("BUILD_randpkt", "OFF")
        .define("BUILD_randpktdump", "OFF")
        .define("BUILD_rawshark", "OFF")
        .define("BUILD_reordercap", "OFF")
        .define("BUILD_sshdump", "OFF")
        .define("BUILD_text2pcap", "OFF")
        .define("BUILD_tfshark", "OFF")
        .define("BUILD_tshark", "OFF")
        .define("BUILD_wifidump", "OFF")
        .define("BUILD_wireshark", "OFF")
        .define("BUILD_xxx2deb", "OFF")
        .build()
}

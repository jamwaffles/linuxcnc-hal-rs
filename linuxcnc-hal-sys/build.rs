extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-Ilinuxcnc-src/src/hal")
        .clang_arg("-Ilinuxcnc-src/src/rtapi")
        // Tell LinuxCNC build to run in non-realtime mode
        // See line ~114 in linuxcnc-src/src/hal/hal.h
        .clang_arg("-DULAPI")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let lcnc_relative_path = "./linuxcnc-src/lib";

    let root = env!("CARGO_MANIFEST_DIR");

    let lcnc_lib_path = Path::new(&root)
        .join(lcnc_relative_path)
        .canonicalize()
        .expect("Failed to canonicalize LinuxCNC search path");

    // Convert shared lib to static lib
    Command::new("ar")
        .args(&[
            "crs",
            &format!("{}/liblinuxcnchal.a", out_path.display()),
            &format!("{}/liblinuxcnchal.so.0", lcnc_lib_path.display()),
        ])
        .status()
        .unwrap();

    // Link to static compiled LinuxCNC `liblinuxcnchal.a` library
    println!("cargo:rustc-link-search=native={}", out_path.display());
    println!("cargo:rustc-link-lib=static=linuxcnchal");
}

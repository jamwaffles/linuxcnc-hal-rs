extern crate bindgen;

use std::env;
// use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=patch/config.h");

    // fs::copy("patch/config.h", "linuxcnc-src/src/config.h")
    //     .expect("Failed to copy config patch file");

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

    cc::Build::new()
        .files(&[
            "linuxcnc-src/src/hal/hal_lib.c",
            "linuxcnc-src/src/rtapi/uspace_ulapi.c",
        ])
        .define("ULAPI", None)
        .include("patch")
        .include("linuxcnc-src/src/hal")
        .include("linuxcnc-src/src/rtapi")
        .include("linuxcnc-src/src")
        .warnings(false)
        .compile("linuxcnchal");
}

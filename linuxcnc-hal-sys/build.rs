extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let lcnc_lib_path = Path::new("linuxcnc-src/lib")
        .canonicalize()
        .expect("Path to LCNC lib folder is not valid");

    // Link to compiled LinuxCNC `liblinuxcnc.hal.so` library
    println!(
        "cargo:rustc-link-search={}",
        lcnc_lib_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=linuxcnchal");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Source files from LinuxCNC include folder. LinuxCNC must be built for this to work - see
        // README.md for details.
        .clang_arg("-Ilinuxcnc-src/include")
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
}

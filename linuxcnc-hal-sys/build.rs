extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // TODO: Use pkg_config to probe for global liblinuxcnchal.so.0, otherwise fall back to
    // LINUXCNC_SRC env var.

    let linuxcnc_root = env::var("LINUXCNC_SRC").expect("LINUXCNC_SRC env var must be set and pointing to the root of the LinuxCNC source Git repository");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(&format!("-I{}", linuxcnc_root))
        .clang_arg(&format!("-I{}/src/hal", linuxcnc_root))
        .clang_arg(&format!("-I{}/src/rtapi", linuxcnc_root))
        // Tell LinuxCNC build to run in realtime mode with `-DRTAPI` or non-realtime with `-DULAPI`.
        // See line ~114 in linuxcnc-src/src/hal/hal.h
        .clang_arg("-DRTAPI")
        // .clang_arg("-DULAPI")
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

    // Dynamically link LinuxCNC HAL, as per <https://github.com/rust-lang/rust-bindgen/issues/1974>
    // and <https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib>

    // Required for non-rt stuff to link properly
    println!("cargo:rustc-link-search=native={linuxcnc_root}/lib");
    println!("cargo:rustc-link-lib=dylib=linuxcnchal");

    // From
    // <https://stackoverflow.com/questions/40602708/linking-rust-application-with-a-dynamic-library-not-in-the-runtime-linker-search>
    println!("cargo:rustc-link-arg=-Wl,-rpath,{linuxcnc_root}/lib");

    // println!("cargo:rustc-link-lib=dylib=rtapi");

    // // TODO: Use CARGO_FEATURE_<name> to enable this for non-rt builds. See more here:
    // // https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
    // // // ONLY required for non-realtime builds
    // let comp = cc::Build::new()
    //     .cpp(true)
    //     .files(&[
    //         &format!("{}/src/hal/hal_lib.c", linuxcnc_root),
    //         &format!("{}/src/rtapi/uspace_ulapi.c", linuxcnc_root),
    //     ])
    //     .flag("-std=gnu++11")
    //     .define("ULAPI", None)
    //     // .include("patch")
    //     .include(&format!("{}/src/hal", linuxcnc_root))
    //     .include(&format!("{}/src/rtapi", linuxcnc_root))
    //     .include(&format!("{}/src", linuxcnc_root))
    //     .warnings(false)
    //     .get_compiler();

    // // panic!("{:?}", comp);

    // comp.compile("linuxcnchal");
}

extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
static EDK_LIBRARY_PATH: &'static str = "bin/linux64";

fn main() {
    println!("cargo:rustc-link-search={}", EDK_LIBRARY_PATH);
    println!("cargo:rustc-link-lib=edk");

    let bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("include/Iedk.h")
        .generate_comments(false)
        .generate()
        .expect("failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap_or_else(|err| {
            panic!("couldn't write bindings: {}", err);
        });
}

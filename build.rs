extern crate cc;
use std::path::PathBuf;

fn main() {
    /* Assume libdrgn.a is exist in the specific path and it to
     * search for static library */
    let path = PathBuf::from("drgn/libdrgn/.libs").canonicalize().unwrap();
    let path = path.to_str().unwrap();
    println!("cargo:rustc-link-search=native={path}");

    // Add shared library(.so)
    println!("cargo:rustc-link-lib=gomp");
    println!("cargo:rustc-link-lib=dw");
    println!("cargo:rustc-link-lib=elf");

    println!("cargo:rerun-if-changed=lib/knight.c");

    cc::Build::new()
        .file("lib/knight.c")
        .include("drgn/libdrgn")
        .compile("knight");
}

extern crate cc;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Build libdrgnimpl with the simple script
    let output = Command::new("scripts/build-drgn.sh")
        .output()
        .expect("Failed to build libdrgnimpl");

    assert!(output.status.success());

    // Add current path to search the static library
    let path = PathBuf::from("build.rs").canonicalize().unwrap();
    let parent = path.parent().unwrap().to_str().unwrap();
    println!("cargo:rustc-link-search=native={parent}");

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

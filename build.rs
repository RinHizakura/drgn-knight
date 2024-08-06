extern crate cc;
extern crate pkg_config;

use std::path::PathBuf;
use std::process::Command;

fn main() {
    /* Build libdrgn.a */
    let output = Command::new("make")
        .arg("libdrgn_a")
        .output()
        .expect("Failed to build libdrgnimpl");

    assert!(output.status.success());

    /* Find libdrgn.a in the specific path */
    let path = PathBuf::from("drgn/libdrgn/.libs").canonicalize().unwrap();
    let path = path.to_str().unwrap();
    println!("cargo:rustc-link-search=native={path}");

    // Add shared library(.so)
    println!("cargo:rustc-link-lib=gomp");
    println!("cargo:rustc-link-lib=dw");
    println!("cargo:rustc-link-lib=elf");
    if pkg_config::probe_library("libkdumpfile").is_ok() {
        println!("cargo:rustc-link-lib=kdumpfile");
    }

    println!("cargo:rerun-if-changed=lib/knight.c");

    cc::Build::new()
        .file("lib/knight.c")
        .include("drgn/libdrgn")
        .compile("knight");
}

extern crate cc;
extern crate pkg_config;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let arch = build_target::target_arch().unwrap();

    /* Build libdrgn.a */
    let outdir = std::env::var_os("OUT_DIR").unwrap();
    let outdir_str = outdir.to_str().unwrap();
    let out_libdrgn = format!("{outdir_str}/libdrgn.a");
    if !Path::new(&out_libdrgn).exists() {
        let output = Command::new("make")
            .arg("libdrgn_a")
            .arg(format!("ARCH={arch}"))
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to build libdrgnimpl");

        println!("{}", String::from_utf8(output.stderr).unwrap());
        assert!(output.status.success());

        // Create a file as footprint
        let output = Command::new("touch")
            .arg(&out_libdrgn)
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to create libdrgn.a");

        println!("{}", String::from_utf8(output.stderr).unwrap());
        assert!(output.status.success());
    }

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

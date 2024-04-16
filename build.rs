//#![allow(dead_code)]
#![allow(unused_must_use)]

use bindgen;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = std::env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();

    #[cfg(target_os = "linux")]
    build_and_statically_link_linux(&out_dir);

    #[cfg(target_os = "windows")]
    build_and_statically_link_windows();

    #[cfg(target_os = "macos")]
    build_and_statically_link_macos();
}

#[cfg(target_os = "linux")]
fn build_and_statically_link_linux(out_dir: &str) {
    let install_path = format!("{}/build", out_dir);
    std::fs::remove_dir_all(&install_path);
    std::fs::create_dir(&install_path);
    // autoreconf -f -i
    Command::new("/usr/bin/autoreconf")
        .current_dir("file/")
        .arg("-f")
        .arg("-i")
        .status()
        .unwrap();

    // make distclean
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .arg("distclean")
        .status();
    // ./configure --disable-silent-rules --enable-static=true --enable-shared=false --prefix=$(pwd)/build CC=x86_64-linux-musl-gcc
    let mut configure_args: Vec<String> = Vec::from([
        "./configure",
        "--disable-silent-rules",
        "--enable-static=true",
        "--enable-shared=false",
        &format!("--prefix={}", &install_path),
    ])
    .into_iter()
    .map(|i| i.to_string())
    .collect();

    if let Ok(target) = std::env::var("TARGET") {
        if target.starts_with("x86_64-unknown-linux-musl") {
            configure_args.push(format!("CC={}", "x86_64-linux-musl-gcc"))
        }
        if target.starts_with("aarch64-unknown-linux-musl") {
            configure_args.push(format!("CC={}", "aarch64-linux-musl-gcc"))
        }
    }
    Command::new("sh")
        .current_dir("file/")
        .args(configure_args)
        .status()
        .unwrap();

    // make -j4
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .arg("-j4")
        .status()
        .unwrap();
    // make -C tests check
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .args(["-C", "tests", "check"])
        .status()
        .unwrap();
    // make install
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .arg("install")
        .status()
        .unwrap();

    println!("cargo:rustc-flags=-l static=magic");
    println!("cargo:rustc-link-search=native={}/lib", &install_path);
    bindgen::Builder::default()
        .header(format!("{}/include/magic.h", &install_path))
        .clang_arg(format!("-I{}/include/", &install_path))
        .allowlist_var("MAGIC_.*")
        .allowlist_function("magic_.*")
        .layout_tests(false)
        .generate()
        .unwrap()
        .write_to_file("src/magic/magic_sys.rs")
        .unwrap();
}

#[cfg(target_os = "windows")]
fn build_and_statically_link_windows() {
    !unimplemented!("windows")
}

#[cfg(target_os = "macos")]
fn build_and_statically_link_macos() {
    !unimplemented!("macos")
}

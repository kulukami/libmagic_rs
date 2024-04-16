//#![allow(dead_code)]
#![allow(unused_must_use)]

use bindgen;
use std::process::Command;

fn main() {
    let path_cur = std::env::current_dir().unwrap().display().to_string();
    build_and_statically_link(&path_cur);

    println!("cargo:rustc-flags=-l static=magic");
    println!("cargo:rustc-link-search=native={}/build/lib", path_cur);
    let builder = bindgen::Builder::default()
        .header(format!("{}/build/include/magic.h", path_cur))
        .clang_arg("-I./installed/include/")
        .allowlist_var("MAGIC_.*")
        .allowlist_function("magic_.*")
        .layout_tests(false)
        .generate()
        .unwrap()
        .write_to_file("src/magic/magic_sys.rs")
        .unwrap();
}

fn build_and_statically_link(build_path: &str) {
    std::fs::remove_dir_all("build");
    std::fs::create_dir("build");
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
    println!("cargo:warning=MESSAGE2");
    // ./configure --disable-silent-rules --enable-static=true --enable-shared=false --prefix=$(pwd)/../build CC=x86_64-linux-musl-gcc
    let mut configure_args: Vec<String> = Vec::from([
        "./configure",
        "--disable-silent-rules",
        "--enable-static=true",
        "--enable-shared=false",
        &format!("--prefix={}/build", &build_path),
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
    println!("cargo:warning=MESSAGE3");

    // make -j4
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .arg("-j4")
        .status()
        .unwrap();
    println!("cargo:warning=MESSAGE4");
    // make -C tests check
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .args(["-C", "tests", "check"])
        .status()
        .unwrap();
    println!("cargo:warning=MESSAGE5");
    // make install
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .arg("install")
        .status()
        .unwrap();
    println!("cargo:warning=MESSAGE6");
}

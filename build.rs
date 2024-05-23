//#![allow(dead_code)]
#![allow(unused_must_use)]

use bindgen;
use std::{collections::HashMap, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = std::env::var_os("OUT_DIR")
        .unwrap()
        .to_string_lossy()
        .to_string();

    if let Ok(target) = std::env::var("TARGET") {
        if target.starts_with("x86_64-unknown-linux-musl")
            || target.starts_with("aarch64-unknown-linux-musl")
        {
            build_and_statically_link_linux(&out_dir);
        } else if target.starts_with("x86_64-pc-windows-gnu") {
            // cross-compile linux to windows
            build_and_statically_link_windows(&out_dir);
        } else if target.starts_with("aarch64-apple-darwin")
            || target.starts_with("x86_64-apple-darwin")
        {
            // cross-compile linux to macos
            build_and_statically_link_macos();
        } else {
            build_and_statically_link_linux(&out_dir)
        }
    }
}

fn apply_patches() {
    Command::new("/usr/bin/patch")
        .current_dir("./")
        .arg("-N")
        .arg("-u")
        .arg("file/Makefile.am")
        .arg("-i")
        .arg("patches/file_Makefile_am.patch")
        .status();

    Command::new("/usr/bin/patch")
        .current_dir("./")
        .arg("-N")
        .arg("-u")
        .arg("file/configure.ac")
        .arg("-i")
        .arg("patches/file_configure_ac.patch")
        .status();
}

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
        "--enable-static=yes",
        "--enable-shared=no",
        &format!("--prefix={}", &install_path),
    ])
    .into_iter()
    .map(|i| i.to_string())
    .collect();

    if let Ok(target) = std::env::var("TARGET") {
        if target.starts_with("x86_64-unknown-linux-musl") {
            configure_args.push(format!("CC={}", "x86_64-linux-musl-gcc"));
        } else if target.starts_with("aarch64-unknown-linux-musl") {
            configure_args.push(format!("CC={}", "aarch64-linux-musl-gcc"));
            configure_args.push(format!("--host=aarch64-pc-linux-gnu"));
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

fn build_and_statically_link_windows(out_dir: &str) {
    let install_path = format!("{}/build", out_dir);
    std::fs::remove_dir_all(&install_path);
    std::fs::create_dir(&install_path);
    let mut configure_args: Vec<String> = Vec::from([
        "./configure",
        "--host=x86_64-w64-mingw32",
        &format!("--prefix={}", &install_path),
    ])
    .into_iter()
    .map(|i| i.to_string())
    .collect();

    Command::new("sh")
        .current_dir("libgnurx/")
        .args(&configure_args)
        .status()
        .unwrap();

    // make -j2
    Command::new("/usr/bin/make")
        .current_dir("libgnurx/")
        .arg("clean")
        .status()
        .unwrap();

    // make -j2
    Command::new("/usr/bin/make")
        .current_dir("libgnurx/")
        .arg("-j2")
        .status()
        .unwrap();

    // make -j2
    Command::new("/usr/bin/make")
        .current_dir("libgnurx/")
        .arg("install")
        .status()
        .unwrap();

    // windows patch
    apply_patches();

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
        "--enable-static=yes",
        "--enable-shared=no",
        "--host=x86_64-w64-mingw32",
        &format!("--prefix={}", &install_path),
    ])
    .into_iter()
    .map(|i| i.to_string())
    .collect();

    let mut win_env: HashMap<String, String> = std::env::vars().collect();

    win_env.insert("CC".to_string(), "x86_64-w64-mingw32-gcc".to_string());
    win_env.insert("CXX".to_string(), "x86_64-w64-mingw32-c++".to_string());
    win_env.insert("CFLAGS".to_string(), format!("-I{}/include", install_path));
    win_env.insert(
        "LDFLAGS".to_string(),
        format!("-L{}/lib -lshlwapi", install_path),
    );

    Command::new("sh")
        .current_dir("file/")
        .envs(&win_env)
        .args(configure_args)
        .status()
        .unwrap();

    // make -j4
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .envs(&win_env)
        .arg("-j4")
        .status()
        .unwrap();

    // make install
    Command::new("/usr/bin/make")
        .current_dir("file/")
        .envs(&win_env)
        .arg("install")
        .status()
        .unwrap();

    println!("cargo:rustc-flags=-l static=magic");
    println!("cargo:rustc-flags=-l static=regex");
    println!("cargo:rustc-link-search=native={}/lib", install_path);
    bindgen::Builder::default()
        .header(format!("{}/include/magic.h", &install_path))
        .clang_arg(format!("-I{}/include", &install_path))
        .clang_arg(format!("-L{}/lib", &install_path))
        .allowlist_var("MAGIC_.*")
        .allowlist_function("magic_.*")
        .layout_tests(false)
        .generate()
        .unwrap()
        .write_to_file("src/magic/magic_sys.rs")
        .unwrap();
}

fn build_and_statically_link_macos() {
    !unimplemented!("macos")
}

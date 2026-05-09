use pb_rs::ConfigBuilder;
use pb_rs::types::FileDescriptor;
use std::env;
use std::fs;

use crate::codegen::gen_cxx_binding;

#[path = "../include/codegen.rs"]
mod codegen;

#[allow(clippy::unwrap_used)]
fn main() {
    println!("cargo:rerun-if-changed=proto/update_metadata.proto");

    gen_cxx_binding("boot-rs");
    build_native_cli();

    let cb = ConfigBuilder::new(
        &["proto/update_metadata.proto"],
        None,
        Some(&"proto"),
        &["."],
    )
    .unwrap();
    FileDescriptor::run(
        &cb.single_module(true)
            .dont_use_cow(true)
            .generate_getters(true)
            .build(),
    )
    .unwrap();
}

fn build_native_cli() {
    if env::var_os("CARGO_FEATURE_NATIVE").is_none() {
        return;
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let generated = std::path::Path::new(&out_dir).join("generated");
    fs::create_dir_all(&generated).unwrap();
    fs::write(
        generated.join("flags.h"),
        "#pragma once\n\
         #define MAGISK_VERSION      \"native\"\n\
         #define MAGISK_VER_CODE     0\n\
         #define MAGISK_DEBUG        1\n",
    )
    .unwrap();

    for file in [
        "boot-rs.cpp",
        "bootimg.cpp",
        "../base/base-rs.cpp",
        "../base/base.cpp",
        "../external/cxx-rs/src/cxx.cc",
    ] {
        println!("cargo:rerun-if-changed={file}");
    }

    cc::Build::new()
        .cpp(true)
        .std("c++2a")
        .define("_GNU_SOURCE", None)
        .flag_if_supported("-fpermissive")
        .include(".")
        .include("../include")
        .include("../base/include")
        .include(&generated)
        .file("boot-rs.cpp")
        .file("bootimg.cpp")
        .file("../base/base-rs.cpp")
        .file("../base/base.cpp")
        .file("../external/cxx-rs/src/cxx.cc")
        .compile("magiskboot-cxx");

    cc::Build::new()
        .include("../external/lz4/lib")
        .file("../external/lz4/lib/lz4.c")
        .file("../external/lz4/lib/lz4frame.c")
        .file("../external/lz4/lib/lz4hc.c")
        .file("../external/lz4/lib/xxhash.c")
        .compile("magiskboot-lz4");
}

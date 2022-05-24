extern crate bindgen;
extern crate cmake;

use cmake::Config;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    
    // CMake
    let _ = Config::new("xgboost")
        .uses_cxx11()
        .define("BUILD_STATIC_LIB", "ON")
        .build();

    // CONFIG BINDGEN
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-x", "c++", "-std=c++11"])
        .clang_arg(format!(
            "-I{}",
            Path::new(&String::from("xgboost/include")).display()
        ))
        .clang_arg(format!(
            "-I{}",
            Path::new(&String::from("xgboost/rabit/include")).display()
        ))
        .clang_arg(format!(
            "-I{}",
            Path::new(&String::from("xgboost/dmlc-core/include")).display()
        ))
        .generate()
        .expect("Unable to generate bindings.");

    // GENERATE THE BINDINGS
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");

    // LINK STUFF (LINUX)
    println!("cargo:rustc-link-search={}", out_path.join("lib").display());
    println!("cargo:rustc-link-lib=xgboost");
    println!("cargo:rustc-link-lib=dmlc");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=gomp");
}

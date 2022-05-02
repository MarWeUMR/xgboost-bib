extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pwd = env::current_dir().unwrap();
    let xgb_root = Path::new(&out_dir).join("xgboost");

    // copy source code into OUT_DIR for compilation if it doesn't exist
    if xgb_root.exists() == false {
        println!("Copying XGBoost source code into OUT_DIR");
        Command::new("cp")
            .args(&["-r", "xgboost", xgb_root.to_str().unwrap()])
            .status()
            .unwrap_or_else(|e| {
                panic!("Failed to copy ./xgboost to {}: {}", xgb_root.display(), e);
            });

        println!("copying done");
        // compile xgboost

        println!("{}/build", xgb_root.to_str().unwrap());

        let output = Command::new("mkdir")
            .arg("-p")
            .arg(format!("{}/build", xgb_root.to_str().unwrap()).as_str())
            .output()
            .unwrap();

        println!("created build dir");
        std::env::set_current_dir(format!("{}/build", xgb_root.to_str().unwrap()))
            .expect("Unable to change into [path to executable]/nvs");
        let o = Command::new("pwd").output().unwrap();
        println!("pwd: \n{:?}", String::from_utf8(o.stdout).unwrap());

        Command::new("cmake")
            .arg("..")
            .status()
            .unwrap_or_else(|e| {
                panic!("cmake error: {}", e);
            });

        Command::new("make")
            .arg("-j16")
            .status()
            .unwrap_or_else(|e| {
                panic!("make error: {}", e);
            });

        let o = Command::new("ls").output().unwrap();
        println!("ls: \n{:?}", String::from_utf8(o.stdout).unwrap());

        std::env::set_current_dir(pwd).expect("Unable to change into [path to executable]/nvs");
    }

    let xgb_root = xgb_root.canonicalize().unwrap();
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&["-x", "c++", "-std=c++11"])
        .clang_arg(format!("-I{}", xgb_root.join("include").display()))
        .clang_arg(format!("-I{}", xgb_root.join("rabit/include").display()))
        .clang_arg(format!(
            "-I{}",
            xgb_root.join("dmlc-core/include").display()
        ))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");

    println!("cargo:rustc-link-search={}", xgb_root.join("lib").display());

    println!(
        "cargo:rustc-link-search={}",
        xgb_root.join("rabit/lib").display()
    );
    println!(
        "cargo:rustc-link-search={}",
        xgb_root.join("dmlc-core").display()
    );
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=xgboost");
}

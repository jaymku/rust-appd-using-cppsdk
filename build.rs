use std::env;
use std::path::PathBuf;

fn main() {
    let sdk_root = "appdynamics-cpp-sdk";

    // 1. Link the AppD C++ SDK (project is built for Linux only, e.g. in Docker)
    println!("cargo:rustc-link-search=native={}/lib", sdk_root);
    println!("cargo:rustc-link-lib=appdynamics");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}/include/appdynamics.h", sdk_root);

    // 2. Bindgen configuration
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg(format!("-I{}/include", sdk_root))
        .allowlist_function("appd_.*")
        .allowlist_type("appd_.*")
        .allowlist_var("APPD_.*")
        .generate()
        .expect("Unable to generate bindings");

    // 3. Output bindings to OUT_DIR
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(&out_path).expect("Couldn't write bindings!");
}

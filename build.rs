use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=cmark");
    bindgen::Builder::default()
        .header("/usr/include/cmark.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false)
        .generate()
        .unwrap()
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .unwrap();
}

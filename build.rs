use std::env;
use std::path::PathBuf;
use bindgen;
use std::path::Path;
use std::fs;


// Returns vector of file pathes found under a certin path
fn find_files(path: &Path) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            let entry_path = entry.unwrap().path();
            if entry_path.is_dir() {
                // Recurse into directories
                files.extend(find_files(&entry_path));
            } else if entry_path.is_file() {
                // Add files to the list
                if let Some(path_str) = entry_path.to_str() {
                    files.push(path_str.to_string());
                }
            }
        }
    } else {
        let path_str = path.to_str().unwrap();
        panic!("Failed to read directory {path_str}");
    }

    files
}


fn main() {
    let roc_path= Path::new("/home/mkh/Coding/roc-toolkit/");
    let roc_ld = roc_path.join("bin/x86_64-pc-linux-gnu");
    let roc_include = roc_path.join("src/public_api/include");
    // Tell cargo to look for shared libraries in the specified directory
    println!("{}", format!("cargo:rustc-link-search={}", roc_ld.to_str().unwrap()));

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=dylib=roc");

    let headers = find_files(roc_include.as_path());

    // Build up options for the  bindings.
    let bindings = bindgen::Builder::default()
        // The input headers we would like to generate
        // bindings for.
        .headers(headers)
        // Serch path
        .clang_arg(format!("-I{}", roc_include.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

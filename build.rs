use std::env;
use std::process::Command;
use std::path::PathBuf;
use bindgen;
use std::path::Path;
use std::fs;

// TODO: ship roc-toolkit in sources to linux, and binaries to macos and windows

/// Function returns vector of file pathes found under a certain path
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

/// Function to call scons
fn call_scons() -> Result<(), Box<dyn std::error::Error>> {
    // Determine the current directory
    let current_dir = "external/roc-toolkit";
    eprintln!("Running scons in {:?}", current_dir);

    // Invoke `scons`
    let status = Command::new("scons")
        .current_dir(&current_dir) // Run scons in the current directory
        .args(["--disable-tools",
            "--build-3rdparty=openfec,speexdsp,libuv,libunwind,openssl,sndfile",
            "--enable-static",
            "--disable-shared"])
        .status()?; // Capture the status of the scons command

    if !status.success() {
        return Err(format!("scons failed with exit code: {:?}", status.code()).into());
    }

    Ok(())
}

/// Check if a tool is available in the system PATH
fn check_tool(tool: &str) {
    if Command::new(tool).arg("--version").output().is_err() {
        eprintln!("Error: {} is not installed or not found in PATH.", tool);
        std::process::exit(1);
    }
}

fn main() {
    // Build roc-toolkit
    let build_tools = ["scons", "ragel", "python3"];
    for bt in build_tools.iter() { check_tool(bt); }
    // Call the `scons` command
    if let Err(e) = call_scons() {
        eprintln!("Error calling scons: {}", e);
        std::process::exit(1);
    }

    let roc_path= Path::new("external/roc-toolkit/");
    let roc_ld = roc_path.join("bin/x86_64-pc-linux-gnu");
    let roc_include = roc_path.join("src/public_api/include");
    // Tell cargo to look for shared libraries in the specified directory
    println!("{}", format!("cargo:rustc-link-search={}", roc_ld.to_str().unwrap()));

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=roc");

    let headers = find_files(roc_include.as_path());

    // Build up options for the  bindings.
    let bindings = bindgen::Builder::default()
        // The input headers we would like to generate
        // bindings for.
        .headers(headers)
        // Search path
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

extern crate reqwest;

use std::env;
use std::fs;
use std::io;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;


fn main() -> Result<(), String> {
    // http://www.libsdl.org/release/SDL2-devel-2.0.16-VC.zip
    let target = env::var("TARGET").unwrap();
    println!("TARGET: {:?}", target);
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        let os_bit: &str;
        if target.contains("x86_64") {
            os_bit = "x64";
        } else {
            os_bit = "x86";
        }
        println!("os_bit: {}", os_bit);
        lib_dir.push("rust-sdl2-libs/lib");
        dll_dir.push("rust-sdl2-libs/lib");
        lib_dir.push(os_bit);
        dll_dir.push(os_bit);

        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    if !new_file_path.as_path().exists(){
                        std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                    }
                }
            }
        }
    }
    Ok(())
}


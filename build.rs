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
        if target.contains("msvc") {
            println!("os_bit: {}", os_bit);
            let version = "2.0.16";
            let lib_file_base_name = format!("SDL2-devel-{}-VC", version);
            let lib_file_name = format!("{}.zip", lib_file_base_name);
            let dll_file_base_name = format!("SDL2-{}-win32-{}", version, os_bit);
            let dll_file_name = format!("{}.zip", dll_file_base_name);
            if !Path::new(&lib_file_name).exists() {
                download_from_sdl_site(&lib_file_name)?;
            }
            if !Path::new(&dll_file_name).exists() {
                download_from_sdl_site(&dll_file_name)?;
            }
            if !Path::new(&lib_file_base_name).exists() {
                extract_zip(&lib_file_name, &lib_file_base_name);
            }
            if !Path::new(&dll_file_base_name).exists() {
                extract_zip(&dll_file_name, &dll_file_base_name);
            }
            lib_dir.push(lib_file_base_name);
            lib_dir.push(format!("SDL2-{}", version));
            dll_dir.push(dll_file_base_name);
        }
        lib_dir.push("lib");
        lib_dir.push(os_bit);

        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir") {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path())
                        .expect("Can't copy from DLL dir");
                }
            }
        }
    }
    Ok(())
}

fn download_from_sdl_site(file_name: &str) -> Result<(), String>{
    let target = format!("http://www.libsdl.org/release/{}", file_name);
    let response = reqwest::blocking::get(target).map_err(|e| e.to_string())?.bytes().map_err(|e| e.to_string())?;
    let mut out =
        File::create(&file_name).expect("failed to create file");
    out.write_all(&response).map_err(|e| e.to_string())?;
    Ok(())
}


fn extract_zip(path: &str, target_path: &str) -> i32 {
    let fname = std::path::Path::new(path);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    let dir_path = Path::new(target_path);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let mut outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        outpath = dir_path.join(outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    return 0;
}
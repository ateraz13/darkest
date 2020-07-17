use std::env;
use std::path::Path;
use std::path::PathBuf;

#[cfg(any(windows))]
fn sdl_setup () {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        lib_dir.push("lib/sdl2");
        dll_dir.push("lib/sdl2");
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("x64");
            dll_dir.push("x64");
        }
        else {
            lib_dir.push("x32");
            dll_dir.push("x32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }
}

#[cfg(any(unix))]
fn sdl_setup () {
}


// #[cfg(any(unix))]
// fn prepare_asset_dir () -> std::io::Result<()> {

//     macro_rules! links {
//         {$target:expr; $($a:expr),+} => {
//             [
//                 $((std::env::current_dir()?.join($a), $target.join($a))),+
//             ]
//         }
//     }

//     let out_dir = env::var_os("OUT_DIR").unwrap().clone();

//     println!("OUT_DIR={:?}", out_dir);

//     let symlinks = links! {
//         Path::new(&out_dir).join("../../../"); "shaders", "assets"
//     };

//     for (a, b) in &symlinks {
//         println!("Checking symlink: {}", b.display());
//         if let Err(_e) = std::fs::read_link(b) {
//             println!("creating symlink: {} -> {}", a.display(), b.display());
//             std::os::unix::fs::symlink(&a, &b)?;
//         }

//     }

//     Ok(())
// }

// #[cfg(any(windows))]
// fn prepare_asset_dir () -> std::io::Result<()> {
//     Ok(())
// }


fn main () -> std::io::Result<()>{
    // We do not need to re-run this script after initial run
    println!("Running build script!");
    println!("cargo:rerun-if-changed=build.rs");
    sdl_setup();
    // prepare_asset_dir()?;
    Ok(())
}

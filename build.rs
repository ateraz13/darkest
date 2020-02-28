use std::env;
use std::path::Path;

#[cfg(any(unix))]
fn link_all_files () -> std::io::Result<()> {

    macro_rules! links {
        {$target:expr; $($a:expr),+} => {
            [
                $((std::env::current_dir()?.join($a), $target.join($a))),+
            ]
        }
    }

    let out_dir = env::var_os("OUT_DIR").unwrap().clone();

    println!("OUT_DIR={:?}", out_dir);

    let symlinks = links! {
        Path::new(&out_dir).join("../../../"); "shaders", "assets"
    };

    for (a, b) in &symlinks {
        println!("Checking symlink: {}", b.display());
        if let Err(_e) = std::fs::read_link(b) {
            println!("creating symlink: {} -> {}", a.display(), b.display());
            std::os::unix::fs::symlink(&a, &b)?;
        }

    }

    Ok(())
}

#[cfg(any(windows))]
fn link_all_files () -> std::io::Result<()> {
    Ok(())
}

fn main () -> std::io::Result<()>{
    // We do not need to re-run this script after initial run
    println!("Running build script!");
    println!("cargo:rerun-if-changed=build.rs");
    link_all_files()?;
    Ok(())
}

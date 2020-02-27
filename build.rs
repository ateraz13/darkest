use std::env;
use std::path::Path;

#[cfg(any(unix))]
fn link_all_files () -> std::io::Result<()> {

    let out_dir = env::var("OUT_DIR").unwrap().clone();
    let shaders_link_path = Path::new(&out_dir).join("shaders");

    if let Err(_e) = std::fs::read_link(&shaders_link_path) {
        println!("Creating symbolic link to shaders directory");
        std::os::unix::fs::symlink(std::env::current_dir()?.join("shaders"), shaders_link_path)?;
    }

    Ok(())
}

#[cfg(any(windows))]
fn link_all_files () -> std::io::Result<()> {
    Ok(())
}

fn main () -> std::io::Result<()>{

    // We do not need to re-run this script after initial run
    println!("cargo:rerun-if-changed=build.rs");
    link_all_files()?;
    Ok(())
}

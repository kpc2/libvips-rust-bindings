// (c) Copyright 2019-2020 OLX
use std::ops::Not;
use std::{env, fs, path};

fn main() {
    let scoop = match env::var("SCOOP") {
        Ok(path) => path,
        Err(_) => env::var("USERPROFILE").unwrap() + "\\scoop",
    };
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_bin = String::from("bin");
    let dest_bin = out_dir + sep() + &src_bin;
    if let Err(err) = copy_dir(
        path::PathBuf::from(&src_bin),
        path::PathBuf::from(&dest_bin),
    ) {
        panic!("Build error: {}; Path: {:?}", err, src_bin)
    }
    println!("cargo:rustc-link-search=native={}", dest_bin);
    println!("cargo:rustc-link-lib=vips");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-search=native={scoop}\\apps\\libvips\\8.10.6\\lib",);
}

fn copy_dir(src: path::PathBuf, dest: path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(&dest)?;
    let contents = fs::read_dir(src)?;
    for content in contents {
        let dir_entry = content?;
        let mut copy = path::PathBuf::from(dest.as_path());
        copy.push(dir_entry.file_name());
        if dir_entry.file_type()?.is_dir().not() {
            fs::copy(dir_entry.path(), copy)?;
        } else {
            copy_dir(dir_entry.path(), copy)?;
        }
    }
    Ok(())
}

fn sep() -> &'static str {
    unsafe { std::str::from_utf8_unchecked(&[path::MAIN_SEPARATOR as u8]) }
}

// (c) Copyright 2019-2020 OLX
fn main() {
    println!("cargo:rustc-link-lib=vips");
    println!("cargo:rustc-link-lib=glib-2.0");
    println!("cargo:rustc-link-lib=gobject-2.0");
    println!("cargo:rustc-link-search=native=D:/KC/programs/scoop/apps/libvips/8.10.6/lib");
}

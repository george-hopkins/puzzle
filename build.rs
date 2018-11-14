extern crate cc;

use cc::Build;

#[cfg(feature = "gd")]
fn configure_gd(build: &mut Build) {
    println!("cargo:rustc-link-lib=gd");
    build.define("HAVE_LIBGD", "1");
}

#[cfg(not(feature = "gd"))]
fn configure_gd(build: &mut Build) {}

fn main() {
    let mut build = Build::new();
    build
        .include(".")
        .file("vendor/src/puzzle.c")
        .file("vendor/src/tunables.c")
        .file("vendor/src/dvec.c")
        .file("vendor/src/cvec.c")
        .file("vendor/src/compress.c")
        .file("vendor/src/vector_ops.c");
    configure_gd(&mut build);
    build.compile("puzzle");
}

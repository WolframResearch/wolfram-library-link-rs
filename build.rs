extern crate bindgen;
#[macro_use]
extern crate lazy_static;

use std::path::PathBuf;
use std::env;

lazy_static! {
    static ref WOLFRAM_INSTALLATION: PathBuf =
        PathBuf::from("/Applications/Mathematica.app/Contents/");

    static ref WOLFRAM_INCLUDE_C: PathBuf = WOLFRAM_INSTALLATION
        .join("SystemFiles/IncludeFiles/C/");
}

fn main() {
    if !WOLFRAM_INSTALLATION.exists() {
        panic!("no Wolfram System exists at '{}'", WOLFRAM_INSTALLATION.display());
    }

    if !WOLFRAM_INCLUDE_C.exists() {
        // NOTE: For WRI developers, if the Mathematica installation at this path is a
        //       prototype / custom Kernel build, it's
        panic!("no Wolfram System includes files exist at '{}'", WOLFRAM_INCLUDE_C.display());
    }

    generate_bindings();
}

fn generate_bindings() {
    let header = WOLFRAM_INCLUDE_C.join("WolframLibrary.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I/{}", WOLFRAM_INCLUDE_C.display()))
        .header(header.display().to_string())
        .generate_comments(true)
        // NOTE: At time of writing this will silently fail to work if you are using a
        //       nightly version of Rust, making the generated bindings almost impossible
        //       to decipher.
        //
        //       Instead, use `$ cargo doc --document-private-items && open target/doc` to
        //       have a look at the generated documentation, which is easier to read and
        //       navigate anyway.
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate Rust bindings to Wolfram LibraryLink using bindgen");

    let filename = "LibraryLink_bindings.rs";
    // OUT_DIR is set by cargo before running this build.rs file. This will be set to a
    // some mangled subdirectory of the "target" directory normally, and will not persist
    // between builds.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(filename);
    bindings.write_to_file(out_path)
        .expect("failed to write Rust bindings with IO error");
}

extern crate bindgen;

use std::path::PathBuf;
use std::env;
use std::ffi::OsStr;

const GENERATED_BINDINGS_FILE: &str = "LibraryLink_bindings.rs";

// lazy_static! {
//     static ref WOLFRAM_INSTALLATION: PathBuf =
//         PathBuf::from("/Applications/Mathematica Base.app/Contents/");

//     static ref WOLFRAM_INCLUDE_C: PathBuf = WOLFRAM_INSTALLATION
//         .join("SystemFiles/IncludeFiles/C/");
// }

fn main() {
    // if !WOLFRAM_INSTALLATION.exists() {
    //     panic!("no Wolfram System exists at '{}'", WOLFRAM_INSTALLATION.display());
    // }

    // if !WOLFRAM_INCLUDE_C.exists() {
    //     // NOTE: For WRI developers, if the Mathematica installation at this path is a
    //     //       prototype / custom Kernel build, it's
    //     panic!("no Wolfram System includes files exist at '{}'", WOLFRAM_INCLUDE_C.display());
    // }

    const ENV_VAR: &str = "WL_LIBRARY_LINK_SYS_HEADER";

    let path = match std::env::var(ENV_VAR) {
        Ok(path) => PathBuf::from(path),
        Err(err) => panic!("wl-library-link-sys: could not get environment variable: \
            {}: {}", ENV_VAR, err),
    };
    if !path.is_file() {
        panic!("wl-library-link-sys: header file does not exist: {}", path.display());
    }
    if !path.is_absolute() {
        panic!("wl-library-link-sys: expected path to header to be absolute: {}",
            path.display())
    }
    generate_bindings(path);
}

fn generate_bindings(header_file: PathBuf) {
    // For the time being there is no reason this shouldn't be "WolframLibrary.h"
    assert_eq!(header_file.file_name().and_then(OsStr::to_str),
               Some("WolframLibrary.h"));

    let header_dir = match header_file.parent() {
        Some(parent) => parent,
        None => panic!("wl-library-link-sys: header path has no parent directory: {}",
            header_file.display()),
    };

    let bindings = bindgen::Builder::default()
        // Add `header_dir` to the list of directories clang will look for header files in
        // TODO: Are there any file path characters this would
        .clang_arg(format!("-I/{}", header_dir.display()))
        .header(header_file.display().to_string())
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

    // OUT_DIR is set by cargo before running this build.rs file. This will be set to a
    // some mangled subdirectory of the "target" directory normally, and will not persist
    // between builds.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap())
        .join(GENERATED_BINDINGS_FILE);
    bindings.write_to_file(out_path)
        .expect("failed to write Rust bindings with IO error");
}

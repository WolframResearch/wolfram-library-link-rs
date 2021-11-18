//! ```cargo
//! [dependencies]
//! # Use an exact, known-good version, since we really don't want small bugs in bindgen to a
//! # be a problem when running `cargo build` for the first time.
//! bindgen = "=0.53.2"
//! # lazy_static = "^1.1"
//! ```

use std::ffi::OsStr;
use std::path::PathBuf;

const ENV_VAR: &str = "WOLFRAM_C_INCLUDES";
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

    let c_includes = match std::env::var(ENV_VAR) {
        Ok(path) => PathBuf::from(path),
        Err(err) => panic!(
            "wl-library-link-sys: could not get environment variable: {}: {}",
            ENV_VAR, err
        ),
    };
    if !c_includes.is_dir() {
        panic!(
            "wl-library-link-sys: header directory does not exist: {}",
            c_includes.display()
        );
    }
    if !c_includes.is_absolute() {
        panic!(
            "wl-library-link-sys: expected path to headers to be absolute: {}",
            c_includes.display()
        )
    }
    generate_bindings(c_includes);
}

fn generate_bindings(c_includes: PathBuf) {
    // For the time being there is no reason this shouldn't be here.
    assert!(c_includes.ends_with("SystemFiles/IncludeFiles/C/"));
    assert!(c_includes.is_dir());
    assert!(c_includes.is_absolute());

    #[rustfmt::skip]
    let bindings = bindgen::builder()
        .header(c_includes.join("WolframLibrary.h").display().to_string())
        .header(c_includes.join("WolframNumericArrayLibrary.h").display().to_string())
        .header(c_includes.join("WolframCompileLibrary.h").display().to_string())
        .generate_comments(true)
        .rustified_non_exhaustive_enum("MNumericArray_Data_Type")
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
    let out_path: PathBuf = std::env::current_dir()
        .expect("failed to get current directory")
        .join(GENERATED_BINDINGS_FILE);

    bindings
        .write_to_file(out_path)
        .expect("failed to write Rust bindings with IO error");
}

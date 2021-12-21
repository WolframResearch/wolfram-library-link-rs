//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! bindgen = "^0.58.1"
//! wolfram-app-discovery = { git = "ssh://git@stash.wolfram.com:7999/~connorg/wolfram-app-discovery.git" }
//! ```

use std::path::PathBuf;

use wolfram_app_discovery::{WolframApp, WolframVersion};

const BINDINGS_FILENAME: &str = "LibraryLink_bindings.rs";

fn main() {
    let app = WolframApp::try_default().expect("unable to locate default Wolfram app");

    let c_includes = app
        .library_link_c_includes_path()
        .expect("unable to get LibraryLink C includes directory");

    generate_bindings(&app, c_includes);
}

fn generate_bindings(app: &WolframApp, c_includes: PathBuf) {
    // For the time being there is no reason this shouldn't be here.
    assert!(c_includes.ends_with("SystemFiles/IncludeFiles/C/"));
    assert!(c_includes.is_dir());
    assert!(c_includes.is_absolute());

    #[rustfmt::skip]
    let bindings = bindgen::builder()
        .header(c_includes.join("WolframLibrary.h").display().to_string())
        .header(c_includes.join("WolframNumericArrayLibrary.h").display().to_string())
        .header(c_includes.join("WolframIOLibraryFunctions.h").display().to_string())
        .header(c_includes.join("WolframImageLibrary.h").display().to_string())
        .header(c_includes.join("WolframSparseLibrary.h").display().to_string())
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // .rustified_non_exhaustive_enum("MNumericArray_Data_Type")
        .constified_enum_module("MNumericArray_Data_Type")
        .constified_enum_module("MNumericArray_Convert_Method")
        .constified_enum_module("MImage_Data_Type")
        .constified_enum_module("MImage_CS_Type")
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

    let version: WolframVersion =
        app.wolfram_version().expect("unable to get WolframVersion");

    let out_path: PathBuf = out_dir()
        .join("wolfram-library-link-sys/generated/")
        .join(&version.to_string())
        .join(wolfram_app_discovery::target_system_id())
        .join(BINDINGS_FILENAME);

    std::fs::create_dir_all(out_path.parent().unwrap())
        .expect("failed to create parent directories for generating bindings file");

    bindings
        .write_to_file(out_path)
        .expect("failed to write Rust bindings with IO error");
}

fn out_dir() -> PathBuf {
    // TODO: Provide a way to override this location using an environment variable.
    std::env::current_dir().expect("unable to get process current working directory")
}

use std::path::PathBuf;

use wolfram_app_discovery::{WolframApp, WolframVersion};

fn main() {
    // Ensure that changes to environment variables checked by wolfram-app-discovery will
    // cause cargo to rebuild the current crate.
    wolfram_app_discovery::config::set_print_cargo_build_script_directives(true);

    // This crate is being built by docs.rs. Skip trying to locate a WolframApp.
    // See: https://docs.rs/about/builds#detecting-docsrs
    if std::env::var("DOCS_RS").is_ok() {
        // Force docs.rs to use the bindings generated for this version / system.
        let bindings_path = make_bindings_path("13.0.0", "MacOSX-x86-64");

        // This environment variable is included using `env!()`. wolfram-library-link-sys
        // will fail to build if it is not set correctly.
        println!(
            "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
            bindings_path.display()
        );

        return;
    }

    let app: Option<WolframApp> = WolframApp::try_default().ok();

    //-----------------------------------------------------------
    // Generate or use pre-generated Rust bindings to LibraryLink
    //-----------------------------------------------------------
    // See docs/Maintenance.md for instructions on how to generate
    // bindings for new WL versions.

    let bindings_path = use_generated_bindings(app.as_ref());

    // let wolfram_version = app
    //     .wolfram_version()
    //     .expect("unable to get Wolfram Language vesion number");
    // let bindings_path = use_pregenerated_bindings(&wolfram_version);

    println!(
        "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
        bindings_path.display()
    );
}

//========================================================================
// Tell `lib.rs` where to find the file containing the WSTP Rust bindings.
//========================================================================

//-----------------------------------
// Bindings generated at compile time
//-----------------------------------

/// Use bindings that we generate now at compile time.
fn use_generated_bindings(app: Option<&WolframApp>) -> PathBuf {
    let c_includes =
        wolfram_app_discovery::build_scripts::library_link_c_includes_directory(app)
            .expect("unable to get LibraryLink C includes directory")
            .into_path_buf();

    println!(
        "cargo:warning=info: generating LibraryLink bindings from: {}",
        c_includes.display()
    );

    let out_path =
        PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("LibraryLink_bindings.rs");

    generate_and_save_bindings_to_file(&c_includes, &out_path);

    out_path
}

/// Note: The definition of this function is copied from
///       scripts/generate_versioned_bindings.rs. Changes to this copy of the function
///       should also be made to the other copy.
fn generate_and_save_bindings_to_file(c_includes: &PathBuf, out_path: &PathBuf) {
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
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate Rust bindings to Wolfram LibraryLink using bindgen");

    //------------------
    // Save the bindings
    //------------------

    std::fs::create_dir_all(out_path.parent().unwrap())
        .expect("failed to create parent directories for generating bindings file");

    bindings
        .write_to_file(out_path)
        .expect("failed to write Rust bindings with IO error");
}

//-----------------------
// Pre-generated bindings
//-----------------------

#[allow(dead_code)]
fn use_pregenerated_bindings(wolfram_version: &WolframVersion) -> PathBuf {
    let system_id =
        wolfram_app_discovery::system_id_from_target(&std::env::var("TARGET").unwrap())
            .expect("unable to get System ID for target system");

    // FIXME: Check that this file actually exists, and generate a nicer error if it
    //        doesn't.

    let bindings_path = make_bindings_path(&wolfram_version.to_string(), system_id);

    println!("cargo:rerun-if-changed={}", bindings_path.display());

    if !bindings_path.is_file() {
        println!(
            "
    ==== ERROR: wolfram-library-link-sys =====

    Rust bindings for Wolfram LibraryLink for target configuration:

        WolframVersion:    {}
        SystemID:          {}

    have not been pre-generated.

    See wolfram-library-link-sys/generated/ for a listing of currently available targets.

    =========================================
            ",
            wolfram_version, system_id
        );
        panic!("<See printed error>");
    }

    bindings_path
}

/// Path (relative to the crate root directory) to the bindings file.
fn make_bindings_path(wolfram_version: &str, system_id: &str) -> PathBuf {
    // Path (relative to the crate root directory) to the bindings file.
    let bindings_path = PathBuf::from("generated")
        .join(wolfram_version)
        .join(system_id)
        .join("LibraryLink_bindings.rs");

    let absolute_bindings_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(&bindings_path);

    absolute_bindings_path
}

use std::path::PathBuf;

use wolfram_app_discovery::{SystemID, WolframVersion};

/// Oldest Wolfram Version that wolfram-library-link is compatible with.
const WOLFRAM_VERSION: WolframVersion = WolframVersion::new(13, 0, 1);

fn main() {
    env_logger::init();

    // Ensure that changes to environment variables checked by wolfram-app-discovery will
    // cause cargo to rebuild the current crate.
    wolfram_app_discovery::config::set_print_cargo_build_script_directives(true);

    // This crate is being built by docs.rs. Skip trying to locate a WolframApp.
    // See: https://docs.rs/about/builds#detecting-docsrs
    if std::env::var("DOCS_RS").is_ok() {
        // Force docs.rs to use the bindings generated for this version / system.
        let bindings_path = make_bindings_path(WOLFRAM_VERSION, SystemID::MacOSX_x86_64);

        // This environment variable is included using `env!()`. wolfram-library-link-sys
        // will fail to build if it is not set correctly.
        println!(
            "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
            bindings_path.display()
        );

        return;
    }

    //-----------------------------------------------------------
    // Generate or use pre-generated Rust bindings to LibraryLink
    //-----------------------------------------------------------
    // See docs/Maintenance.md for instructions on how to generate
    // bindings for new WL versions.

    let bindings_path = use_pregenerated_bindings();

    println!(
        "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
        bindings_path.display()
    );
}

//========================================================================
// Tell `lib.rs` where to find the file containing the WSTP Rust bindings.
//========================================================================

//-----------------------
// Pre-generated bindings
//-----------------------

fn use_pregenerated_bindings() -> PathBuf {
    let system_id = SystemID::try_from_rust_target(&std::env::var("TARGET").unwrap())
        .expect("unable to get System ID for target system");

    // FIXME: Check that this file actually exists, and generate a nicer error if it
    //        doesn't.

    let bindings_path = make_bindings_path(WOLFRAM_VERSION, system_id);

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
            WOLFRAM_VERSION, system_id
        );
        panic!("<See printed error>");
    }

    bindings_path
}

/// Path (relative to the crate root directory) to the bindings file.
fn make_bindings_path(wolfram_version: WolframVersion, system_id: SystemID) -> PathBuf {
    // Path (relative to the crate root directory) to the bindings file.
    let bindings_path = PathBuf::from("generated")
        .join(&wolfram_version.to_string())
        .join(system_id.as_str())
        .join("LibraryLink_bindings.rs");

    println!(
        "cargo:warning=info: using LibraryLink bindings from: {}",
        bindings_path.display()
    );

    let absolute_bindings_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(&bindings_path);

    absolute_bindings_path
}

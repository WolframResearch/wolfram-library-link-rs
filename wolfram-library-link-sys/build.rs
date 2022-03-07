use std::path::PathBuf;

use wolfram_app_discovery::WolframApp;

fn main() {
    // Ensure that changes to environment variables checked by wolfram-app-discovery will
    // cause cargo to rebuild the current crate.
    wolfram_app_discovery::config::set_print_cargo_build_script_instructions(true);

    // This crate is being built by docs.rs. Skip trying to locate a WolframApp.
    // See: https://docs.rs/about/builds#detecting-docsrs
    if std::env::var("DOCS_RS").is_ok() {
        // Force docs.rs to use the bindings generated for this version / system.
        let bindings_path = make_bindings_path("13.0.0", "MacOSX-x86-64");

        // This environment variable is included using `env!()`. wstp-sys will fail to
        // build if it is not set correctly.
        println!(
            "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
            bindings_path.display()
        );

        return;
    }


    let app = WolframApp::try_default().expect("unable to locate WolframApp");

    //---------------------------------------------------------------
    // Choose the pre-generated bindings to use for the target system
    //---------------------------------------------------------------
    // See docs/Maintenance.md for instructions on how to generate
    // bindings for new WL versions.

    let wolfram_version = app
        .wolfram_version()
        .expect("unable to get Wolfram Language vesion number");
    let system_id =
        wolfram_app_discovery::system_id_from_target(&std::env::var("TARGET").unwrap())
            .expect("unable to get System ID for target system");

    // FIXME: Check that this file actually exists, and generate a nicer error if it
    //        doesn't.

    let bindings_path = make_bindings_path(&wolfram_version.to_string(), system_id);

    println!("cargo:rerun-if-changed={}", bindings_path.display());

    let absolute_bindings_path =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(&bindings_path);

    if !absolute_bindings_path.is_file() {
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

    println!(
        "cargo:rustc-env=CRATE_WOLFRAM_LIBRARYLINK_SYS_BINDINGS={}",
        bindings_path.display()
    );
}

/// Path (relative to the crate root directory) to the bindings file.
fn make_bindings_path(wolfram_version: &str, system_id: &str) -> PathBuf {
    // Path (relative to the crate root directory) to the bindings file.
    let bindings_path = PathBuf::from("generated")
        .join(wolfram_version)
        .join(system_id)
        .join("LibraryLink_bindings.rs");

    bindings_path
}

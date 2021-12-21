use std::path::PathBuf;

use wolfram_app_discovery::WolframApp;

fn main() {
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

    // Path (relative to the crate root directory) to the bindings file.
    let bindings_path = PathBuf::from("generated")
        .join(&wolfram_version.to_string())
        .join(system_id)
        .join("LibraryLink_bindings.rs");

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

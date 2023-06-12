//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! clap = { version = "4.3.3", features = ["derive"] }
//! bindgen = "^0.58.1"
//! wolfram-app-discovery = "0.4.7"
//! ```

use std::path::{Path, PathBuf};

use clap::Parser;

use wolfram_app_discovery::{SystemID, WolframApp, WolframVersion};

const BINDINGS_FILENAME: &str = "LibraryLink_bindings.rs";

#[derive(Parser)]
struct Cli {
    /// Target to generate bindings for.
    #[arg(long)]
    target: Option<String>,
}

fn main() {
    let Cli { target } = Cli::parse();

    let app = WolframApp::try_default().expect("unable to locate default Wolfram app");

    let wolfram_version: WolframVersion =
        app.wolfram_version().expect("unable to get WolframVersion");

    let c_includes = app
        .library_link_c_includes_directory()
        .expect("unable to get LibraryLink C includes directory");

    let targets: Vec<&str> = match target {
        Some(ref target) => vec![target.as_str()],
        None => determine_targets().to_vec(),
    };

    println!("Generating bindings for: {targets:?}");

    for target in targets {
        generate_bindings(&wolfram_version, &c_includes, target);
    }
}

/// Generte bindings for multiple targets at once, based on the current
/// operating system.
fn determine_targets() -> &'static [&'static str] {
    if cfg!(target_os = "macos") {
        &["x86_64-apple-darwin", "aarch64-apple-darwin"]
    } else if cfg!(target_os = "windows") {
        &["x86_64-pc-windows-msvc"]
    } else if cfg!(target_os = "linux") {
        &["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"]
    } else {
        panic!("unsupported operating system for determining LibraryLink bindings target architecture")
    }
}

fn generate_bindings(wolfram_version: &WolframVersion, c_includes: &Path, target: &str) {
    // For the time being there is no reason this shouldn't be here.
    assert!(c_includes.ends_with("SystemFiles/IncludeFiles/C/"));
    assert!(c_includes.is_dir());
    assert!(c_includes.is_absolute());

    let clang_args = vec!["-target", target];

    let target_system_id = SystemID::try_from_rust_target(target)
        .expect("Rust target doesn't map to a known SystemID");

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
        .clang_args(clang_args)
        .generate()
        .expect("unable to generate Rust bindings to Wolfram LibraryLink using bindgen");

    let out_path: PathBuf = out_dir()
        .join("wolfram-library-link-sys/generated/")
        .join(&wolfram_version.to_string())
        .join(target_system_id.as_str())
        .join(BINDINGS_FILENAME);

    std::fs::create_dir_all(out_path.parent().unwrap())
        .expect("failed to create parent directories for generating bindings file");

    bindings
        .write_to_file(&out_path)
        .expect("failed to write Rust bindings with IO error");

    println!("OUTPUT: {}", out_path.display());
}

fn out_dir() -> PathBuf {
    // TODO: Provide a way to override this location using an environment variable.
    std::env::current_dir().expect("unable to get process current working directory")
}

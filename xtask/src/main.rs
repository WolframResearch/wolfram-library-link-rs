//! `cargo xtask` helper commands for the wolfram-library-link-rs project.
//!
//! This crate follows the [`cargo xtask`](https://github.com/matklad/cargo-xtask)
//! convention.

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use wolfram_app_discovery::{SystemID, WolframApp, WolframVersion};

const BINDINGS_FILENAME: &str = "LibraryLink_bindings.rs";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and save LibraryLink bindings for the current platform.
    GenBindings {
        /// Target to generate bindings for.
        #[arg(long)]
        target: Option<String>,
    },
}

//======================================
// Main
//======================================

fn main() {
    let Cli {
        command: Commands::GenBindings { target },
    } = Cli::parse();

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
        .clang_args(clang_args)
        .generate()
        .expect("unable to generate Rust bindings to Wolfram LibraryLink using bindgen");

    let out_path: PathBuf = repo_root_dir()
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

fn repo_root_dir() -> PathBuf {
    let xtask_crate = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    assert!(xtask_crate.file_name().unwrap() == "xtask");
    xtask_crate.parent().unwrap().to_path_buf()
}

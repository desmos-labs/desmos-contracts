use std::{
    path::{Path, PathBuf},
    {fs, process},
    ffi::OsStr,
    sync::atomic::AtomicBool,
    process::exit
};
use core::sync::atomic;
use prost_build::Config;
use log::info;

static QUIET: AtomicBool = AtomicBool::new(false);

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "../desmos";

const DESMOS_GENERATED_PROTO_DIR: &str = "../desmos-cw/proto/src";

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Execute a git cmd with the given appended args
fn run_git_cmd(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("missing exist status");

    if !exit_status.success() {
       panic!("git exited with error code: {:?}", exit_status.code())
    }
}

/// Update the Desmos core submodule with the latest changes of the repository
fn update_desmos_submodule() {
    info!("Updating desmos-labs/desmos submodule...");
    run_git_cmd(&["submodule", "update", "--init"]);
    run_git_cmd(&["-C", DESMOS_DIR, "submodule", "update", "--remote"]);
    // run_git_cmd(&["-C", DESMOS_DIR, "reset", "--hard", "v1.0.0"]); use this if a specific Desmos version is needed
}

/*
/// Walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}
*/

/// Build all the desmos x/profiles module's proto files
fn compile_desmos_profiles_proto(out_dir: &Path) {
    info!(
        "Compiling x/profiles .proto to Rust into '{}'...",
        out_dir.display()
    );

    let desmos_dir = Path::new(DESMOS_DIR);

    let proto_includes_paths = [
        format!("{}/third_party/proto", desmos_dir.display())
    ];

    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    let proto_paths = [
      format!("{}/proto/desmos/profiles/v1beta1/models_profile.proto", desmos_dir.display())
    ];

    // Compile the x/profiles proto files
    Config::new()
        .out_dir(out_dir)
        .compile_protos(&proto_paths, &proto_includes_paths)
        .unwrap();

    info!("Proto files compiled correctly!")
}

fn main() {
    let proto_dir: PathBuf = DESMOS_GENERATED_PROTO_DIR.parse().unwrap();

    info!(
        "Starting the compilation of Desmos .proto files...",
    );

    update_desmos_submodule();
    compile_desmos_profiles_proto(&proto_dir);
}

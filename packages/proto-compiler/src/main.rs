use std::{path::{Path, PathBuf}, ffi::OsStr, process};
use core::sync::atomic;
use prost_build::Config;

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "packages/desmos";

const DESMOS_GENERATED_PROTO_DIR: &str = "packages/desmos-proto/src";

/// Execute a git cmd with the given appended args
fn run_git_cmd(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let exit_status = process::Command::new("git")
        .args(args)
        .status()
        .expect("missing exist status");

    if !exit_status.success() {
       panic!("git exited with error code: {:?}", exit_status.code())
    }
}

/// Update the Desmos core submodule with the latest changes of the repository
fn update_desmos_submodule() {
    println!("Updating desmos-labs/desmos submodule...");

    run_git_cmd(&["submodule", "update", "--init"]);
    run_git_cmd(&["-C", DESMOS_DIR, "submodule", "update", "--remote"]);
    // run_git_cmd(&["-C", DESMOS_DIR, "reset", "--hard", "v1.0.0"]); use this if a specific Desmos version is needed
}

/// Build all the desmos x/profiles module's proto files
fn compile_desmos_profiles_proto(out_dir: &Path) {
    let desmos_dir = Path::new(DESMOS_DIR);
    let generated_profiles_dir = out_dir.join("profiles");

    let proto_includes_paths = [
        desmos_dir.join("proto"),
        desmos_dir.join("third_party/proto")
    ];

    let proto_paths = [
        desmos_dir.join("proto/desmos/profiles/v1beta1/models_profile.proto"),
        desmos_dir.join("proto/desmos/profiles/v1beta1/models_chain_links.proto"),
        desmos_dir.join("proto/desmos/profiles/v1beta1/models_app_links.proto"),
    ];

    // Compile the x/profiles proto files
    Config::new()
        .out_dir(generated_profiles_dir)
        .compile_protos(&proto_paths, &proto_includes_paths)
        .unwrap();

    println!("Proto files compiled correctly!")
}

fn main() {
    let proto_dir: PathBuf = DESMOS_GENERATED_PROTO_DIR.parse().unwrap();

    println!(
        "Starting the compilation of Desmos .proto files...",
    );

    update_desmos_submodule();
    compile_desmos_profiles_proto(&proto_dir);
}

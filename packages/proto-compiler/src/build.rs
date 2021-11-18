use std::path::{Path, PathBuf};
use std::{fs, process};
use std::ffi::OsStr;
use std::sync::atomic::AtomicBool;
use core::sync::atomic;
use std::process::exit;


static QUIET: AtomicBool = AtomicBool::new(false);

/// Directory where the desmos submodule is located
const DESMOS_DIR: &str = "../desmos";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

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

fn update_desmos_submodule() {
    info!("Updating desmos-labs/desmos submodule...");
    run_git_cmd(&["submodule", "update", "--init"]);
    run_git_cmd(&["-C", DESMOS_DIR, "submodule", "update", "--remote"]);
}

fn compile_desmos_profiles_proto(out_dir: &Path) {
    let proto_paths = [
        format!("../desmos/proto/posts/v1beta1")
    ];
    prost_build::compile_protos(&[""])
}



fn main() {

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = DESMOS_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    fs::create_dir(tmp_build_dir.clone()).unwrap();

    info!(
        "Compiling desmos .proto files to Rust into '{}'...",
        out_dir.display()
    )






}

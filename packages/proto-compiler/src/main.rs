use std::{
    ffi::OsStr,
    fs,
    io::Result,
    path::{Path, PathBuf},
    process,
};

/// Directory where the desmos submodule and proto files are located
const DESMOS_DIR: &str = "packages/desmos";
const PROFILES_PROTO_DIR: &str = "proto/desmos/profiles/v1beta1";
const DESMOS_GENERATED_PROTO_DIR: &str = "packages/desmos-proto/src";
const PROFILES_GENERATED_DIR: &str = "profiles";

/// Build all the desmos x/profiles module's proto files
#[cfg(not(tarpaulin_include))]
fn compile_desmos_profiles_proto(out_dir: &Path) -> Result<()> {
    let desmos_submodule_dir = Path::new(DESMOS_DIR);
    let generated_profiles_dir = out_dir.join(PROFILES_GENERATED_DIR);

    let proto_includes_paths = [
        desmos_submodule_dir.join("proto"),
        desmos_submodule_dir.join("third_party/proto"),
    ];

    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    let profiles_proto_dir = desmos_submodule_dir.join(PROFILES_PROTO_DIR);

    let proto_paths = [
        profiles_proto_dir.join("models_profile.proto"),
        profiles_proto_dir.join("models_chain_links.proto"),
        profiles_proto_dir.join("models_app_links.proto"),
        profiles_proto_dir.join("models_dtag_requests.proto"),
        profiles_proto_dir.join("models_relationships.proto"),
    ];

    // Compile the x/profiles proto files
    prost_build::Config::new()
        .btree_map(&["."])
        .out_dir(generated_profiles_dir.clone())
        .compile_protos(&proto_paths, &includes)
        .unwrap();

    println!("Proto files compiled correctly!");

    remove_third_party_files(generated_profiles_dir.as_path())?;
    Ok(())
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let proto_dir: PathBuf = DESMOS_GENERATED_PROTO_DIR.parse().unwrap();

    println!("Starting the compilation of Desmos .proto files...",);

    update_desmos_submodule(DESMOS_DIR);
    compile_desmos_profiles_proto(&proto_dir)?;
    Ok(())
}

/// Execute a git cmd with the given appended args
#[cfg(not(tarpaulin_include))]
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
#[cfg(not(tarpaulin_include))]
fn update_desmos_submodule(desmos_dir: &str) {
    println!("Updating desmos-labs/desmos submodule...");

    run_git_cmd(&["submodule", "update", "--init"]);
    run_git_cmd(&["-C", desmos_dir, "submodule", "update", "--remote"]);
    // run_git_cmd(&["-C", DESMOS_DIR, "reset", "--hard", "v1.0.0"]); use this if a specific Desmos version is needed
}

/// Remove the already provided third_party files from the compiled folders
#[cfg(not(tarpaulin_include))]
fn remove_third_party_files(out_dir: &Path) -> Result<()> {
    fs::remove_file(out_dir.join("cosmos_proto.rs"))?;
    fs::remove_file(out_dir.join("gogoproto.rs"))?;
    fs::remove_file(out_dir.join("google.protobuf.rs"))?;
    Ok(())
}

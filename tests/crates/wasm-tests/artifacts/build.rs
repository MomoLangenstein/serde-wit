use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    std::env::remove_var("CARGO_ENCODED_RUSTFLAGS");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .current_dir("../../wasi_snapshot_preview1")
        .arg("--target=wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &out_dir);
    let status = cmd.status().unwrap();
    assert!(status.success());
    println!("cargo:rerun-if-changed=../../wasi_snapshot_preview1");
    let wasi_adapter = out_dir.join("wasm32-unknown-unknown/release/wasi_snapshot_preview1.wasm");

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .current_dir("../../resource")
        .arg("--target=wasm32-unknown-unknown")
        .env("CARGO_TARGET_DIR", &out_dir);
    let status = cmd.status().unwrap();
    assert!(status.success());
    println!("cargo:rerun-if-changed=../../resource");
    let resource_adapter = out_dir.join("wasm32-unknown-unknown/release/resource.wasm");

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .current_dir("../../wasm-tests")
        .arg("--release")
        .arg("--target=wasm32-wasi")
        .env("CARGO_TARGET_DIR", &out_dir)
        .env("CARGO_PROFILE_DEV_DEBUG", "1");
    let status = cmd.status().unwrap();
    assert!(status.success());

    let mut wasms = Vec::new();
    for file in out_dir.join("wasm32-wasi/release").read_dir().unwrap() {
        let file = file.unwrap().path();
        if file.extension().and_then(|s| s.to_str()) != Some("wasm") {
            continue;
        }

        let dep_file = file.with_extension("d");
        let Ok(deps) = fs::read_to_string(&dep_file) else { continue };//.expect("failed to read dep file");
        for dep in deps.split_once(':').unwrap().1.split_whitespace() {
            println!("cargo:rerun-if-changed={}", dep);
        }

        wasms.push(file);
    }
    println!("cargo:rerun-if-changed=../../wasm-tests/Cargo.toml");

    let mut src = format!("pub const ADAPTER: &str = {wasi_adapter:?};\n");
    src.write_fmt(format_args!("pub const RESOURCE: &str = {resource_adapter:?};\n")).unwrap();

    for wasm in wasms {
        let name = wasm
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_uppercase()
            .replace('-', "_");
        src.write_fmt(format_args!("pub const WASM_{name}: &str = {wasm:?};\n"))
            .unwrap();
    }

    std::fs::write(out_dir.join("wasms.rs"), src).unwrap();
}

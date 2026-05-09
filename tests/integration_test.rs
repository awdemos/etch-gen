use std::process::Command;
use std::path::Path;

#[test]
fn test_preset_fast_test_generates_compiling_crate() {
    let output_dir = tempfile::tempdir().unwrap();
    let output_path = output_dir.path();

    let status = Command::new("cargo")
        .args([
            "run", "--release", "--",
            "preset", "fast-test",
            "--output-dir", output_path.to_str().unwrap(),
        ])
        .current_dir("/var/home/a/code/etch-gen")
        .status()
        .expect("failed to run etch-gen");

    assert!(status.success(), "etch-gen preset command failed");

    let crate_dir = output_path.join("fast_test");
    assert!(crate_dir.exists(), "generated crate directory not found");
    assert!(crate_dir.join("Cargo.toml").exists(), "Cargo.toml not found");
    assert!(crate_dir.join("src/lib.rs").exists(), "lib.rs not found");
    assert!(crate_dir.join("src/main.rs").exists(), "main.rs not found");

    let check_status = Command::new("cargo")
        .args(["check"])
        .current_dir(&crate_dir)
        .status()
        .expect("failed to run cargo check");

    assert!(check_status.success(), "generated crate failed cargo check");
}

#[test]
fn test_preset_mainnet_generates_compiling_crate() {
    let output_dir = tempfile::tempdir().unwrap();
    let output_path = output_dir.path();

    let status = Command::new("cargo")
        .args([
            "run", "--release", "--",
            "preset", "mainnet",
            "--output-dir", output_path.to_str().unwrap(),
        ])
        .current_dir("/var/home/a/code/etch-gen")
        .status()
        .expect("failed to run etch-gen");

    assert!(status.success(), "etch-gen preset command failed");

    let crate_dir = output_path.join("mainnet");
    assert!(crate_dir.exists(), "generated crate directory not found");

    let check_status = Command::new("cargo")
        .args(["check"])
        .current_dir(&crate_dir)
        .status()
        .expect("failed to run cargo check");

    assert!(check_status.success(), "generated crate failed cargo check");
}

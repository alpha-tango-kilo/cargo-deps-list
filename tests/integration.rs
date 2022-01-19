use std::process::Command;

#[test]
fn this_crate() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo-deps-list");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.as_ref().lines().count(), 5);
    assert!(stdout.ends_with("3\n"));
    assert!(output.status.success());
}

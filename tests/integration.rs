use std::process::Command;

#[test]
fn this_crate() {
    let output = Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run cargo-deps-list");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.as_ref().lines().count(),
        1,
        "more/less output than expected"
    );
    assert!(
        stdout.ends_with("Total dependencies: 0\n"),
        "incorrect total dependencies"
    );
    assert!(output.status.success());
}

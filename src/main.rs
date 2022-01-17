use anyhow::{bail, Context, Result};
use itertools::Itertools;
use lazy_regex::{regex, Lazy, Regex};
use std::process::Command;

/*
Finds a single alphanumeric character, then any lowercase letter, digit, or
dash and unlimited number of times. Eventually there'll be a space and the
letter v, followed by a semver. Yes 177 of the 194 characters in this regex
are to parse semver. Deal with it
 */
static CRATE_NAME_AND_VER: &Lazy<Regex> = regex!(
    r#"[a-z]([0-z]|-)* v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"#
);

fn main() -> Result<()> {
    let cargo_tree = Command::new("cargo")
        .args(["tree", "-e", "normal"])
        .output()
        .context("failed to run cargo")?;

    if !cargo_tree.status.success() {
        bail!(
            "cargo command errored (exit code {}):\n{}",
            cargo_tree.status.code().unwrap_or_default(),
            String::from_utf8_lossy(&cargo_tree.stderr),
        );
    }

    let stdout = String::from_utf8_lossy(&cargo_tree.stdout);
    let mut count = 0usize;

    CRATE_NAME_AND_VER
        .captures_iter(stdout.as_ref())
        .map(|capture| capture.get(0).unwrap().as_str())
        .unique()
        .for_each(|dep| {
            count += 1;
            eprintln!("{}", dep);
        });

    eprint!("\nTotal dependencies: ");
    println!("{count}");
    Ok(())
}

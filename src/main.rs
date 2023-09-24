use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};

fn main() {
    if let Err(why) = _main() {
        eprintln!("{why}");
        exit(1);
    }
}

fn _main() -> Result<(), Box<dyn Error>> {
    let cargo_tree = Command::new("cargo")
        .args(["tree", "--prefix", "none", "--format", "{p} {{{f}}}"])
        .args(env::args_os().skip_while(|arg| arg_is_binary_name(arg)))
        .output()
        .map_err(|err| match err.kind() {
            // Just says 'program not found' otherwise
            io::ErrorKind::NotFound => {
                io::Error::new(io::ErrorKind::NotFound, "Cargo not found")
            }
            _ => err,
        })?;

    if !cargo_tree.status.success() {
        return Err(String::from_utf8_lossy(&cargo_tree.stderr).into());
    }

    let cargo_tree_stdout = String::from_utf8_lossy(&cargo_tree.stdout);

    let mut deduplicator = HashSet::new();
    let mut stdout = io::stdout().lock();
    let count = cargo_tree_stdout
        .as_ref()
        .lines()
        // Skip the binary itself
        .skip(1)
        // Strips out trailing things like "(*)"
        .map(|line| match line.find('(') {
            Some(index) => &line[..index - 1],
            None => line,
        })
        // Strips out crates with no enabled features
        .map(|line| line.trim_end_matches(" {}"))
        .filter(|line| deduplicator.insert(*line))
        .map(|dep| {
            stdout.write_all(dep.as_bytes()).unwrap();
        })
        .count();

    writeln!(stdout, "\nTotal dependencies: {count}").unwrap();
    Ok(())
}

/*
Predicate to filter out anything from env::args_os() that is either:
- the binary name (cargo-deps-list)
- cargo
- the cargo subcommand (deps)
 */
fn arg_is_binary_name(arg: &OsStr) -> bool {
    arg.eq_ignore_ascii_case("deps-list")
        || Path::new(arg).file_stem().map_or(false, |name| {
            name.eq_ignore_ascii_case("cargo")
                || name.eq_ignore_ascii_case(env!("CARGO_PKG_NAME"))
        })
}

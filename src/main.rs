use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
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
    let mut deps = cargo_tree_stdout
        .as_ref()
        .lines()
        // Skip the binary itself
        .skip(1)
        // Strips out trailing things like "(*)" and paths
        .map(|line| {
            line.find('(')
                .map(|index| &line[..index - 1])
                .unwrap_or(line)
        })
        // Strips out crates with no enabled features
        .map(|line| line.trim_end_matches(" {}"))
        .filter(|line| deduplicator.insert(*line))
        .collect::<Vec<_>>();
    deps.sort_unstable();

    println!(
        "{deps}{newline}Total dependencies: {count}",
        deps = deps.join("\n"),
        // Pad total with newline if there were any dependencies
        newline = if !deps.is_empty() { "\n" } else { "" },
        count = deps.len(),
    );
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

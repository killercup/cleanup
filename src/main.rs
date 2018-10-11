#[macro_use]
extern crate quicli;
#[macro_use]
extern crate output;

use quicli::prelude::*;
use std::fs::remove_dir_all;
use std::path::Path;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
    /// Directories to scan
    #[structopt(parse(from_os_str))]
    dirs: Vec<PathBuf>,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    debug!("hi there!");
    for dir in &args.dirs {
        if !dir.exists() {
            warn!("Directory {} doesn't exist!", dir.display());
            continue;
        }
        let dir = dir
            .canonicalize()
            .with_context(|_| format!("can't normalize path {}", dir.display()))?;
        dir.read_dir()?
            .flat_map(|x| x)
            .par_bridge()
            .map(|x| x.path())
            .filter(|p| p.is_dir())
            .for_each(|subdir| clean(&subdir));
    }
});

fn clean(dir: &Path) {
    debug!("let's clean {}", dir.display());
    if dir.join("Cargo.toml").exists() {
        clean_cargo(dir);
    }
    if dir.join("package.json").exists() {
        clean_npm(dir);
    }
}

fn clean_cargo(dir: &Path) {
    let target_dir = dir.join("target");
    trace!("would delete {}", target_dir.display());
    if !target_dir.exists() {
        debug!(
            "{} is a cargo dir, but {} doesn't exist",
            dir.display(),
            target_dir.display()
        );
    }
    // remove_dir_all(target_dir);
}

fn clean_npm(dir: &Path) {
    let target_dir = dir.join("node_modules");
    trace!("would delete {}", target_dir.display());
    if !target_dir.exists() {
        debug!(
            "{} is a npm, dir, but {} doesn't exist",
            dir.display(),
            target_dir.display()
        );
    }
    // remove_dir_all(target_dir);
}

/*
 * -----------------------------------------------------------------------------
 * "THE BEER-WARE LICENSE" (Revision 42):
 * <lukic.djordje@gmail.com> wrote this file.  As long as you retain this notice
 * you can do whatever you want with this stuff. If we meet some day, and you
 * think this stuff is worth it, you can buy me a beer in return.  Djordje Lukic
 * -----------------------------------------------------------------------------
 */
use std::env;
use std::os::unix::fs;
use std::path::Path;

enum Paths {
    Both,
    LeftMissing,
    RightMissing,
    BothMissing,
}

fn check_paths(left: &str, right: &str) -> Paths {
    let left_exists = Path::new(left).exists();
    let right_exists = Path::new(right).exists();

    if left_exists && right_exists {
        return Paths::Both;
    }
    if !left_exists && right_exists {
        return Paths::LeftMissing;
    }
    if left_exists && !right_exists {
        return Paths::RightMissing;
    }

    return Paths::BothMissing;
}

fn link(a: &str, b: &str) -> std::io::Result<()> {
    fs::symlink(a, b)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match check_paths(&args[1], &args[2]) {
        Paths::Both => eprint!("Both paths present, don't know what to do"),
        Paths::LeftMissing => link(&args[2], &args[1])?,
        Paths::RightMissing => link(&args[1], &args[2])?,
        Paths::BothMissing => eprint!("Both paths missing, don't know what to do"),
    }

    Ok(())
}

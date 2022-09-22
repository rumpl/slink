/*
 * -----------------------------------------------------------------------------
 * "THE BEER-WARE LICENSE" (Revision 42):
 * <lukic.djordje@gmail.com> wrote this file.  As long as you retain this notice
 * you can do whatever you want with this stuff. If we meet some day, and you
 * think this stuff is worth it, you can buy me a beer in return.  Djordje Lukic
 * -----------------------------------------------------------------------------
 */
use clap::Parser;
use std::error::Error;
use std::os::unix::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(author = "Djordje Lukic", version, about = "Link like a boss")]
struct Arguments {
    #[clap()]
    left: String,

    #[clap()]
    right: String,
}

enum Paths {
    Both,
    LeftMissing,
    RightMissing,
    BothMissing,
}

fn check_paths(left: &str, right: &str) -> Paths {
    match (Path::new(left).exists(), Path::new(right).exists()) {
        (true, true) => Paths::Both,
        (false, true) => Paths::LeftMissing,
        (true, false) => Paths::RightMissing,
        (false, false) => Paths::BothMissing,
    }
}

fn link(a: &str, b: &str) -> std::io::Result<()> {
    fs::symlink(a, b)?;
    println!("Link {} -> {} created", b, a);
    Ok(())
}

fn run(a: &str, b: &str) -> Result<(), Box<dyn Error>> {
    match check_paths(a, b) {
        Paths::Both => Err("Both paths present, don't know what to do".into()),
        Paths::LeftMissing => Ok(link(b, a)?),
        Paths::RightMissing => Ok(link(a, b)?),
        Paths::BothMissing => Err("Both paths missing, don't know what to do".into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Arguments::parse();

    if let Err(e) = run(&arguments.left, &arguments.right) {
        println!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

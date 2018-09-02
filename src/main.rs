mod gittools;
use gittools::{Repository, find_branches, look_at_branches};

use std::path::Path;
use std::fs;

fn run() -> Result<(), gittools::Error> {
    let repo = Repository::open("/home/pal/rust_again/renamee")?;
    let branches = find_branches(&repo)?;
    look_at_branches(&repo, &branches, do_in_branch)?;
    Ok(())
}

fn do_in_branch(branch: &str, workdir: &Path) -> Result<(), gittools::Error> {
    println!("---- Branch: {}", branch);
    let iter = match fs::read_dir(workdir) {
        Ok(val) => val,
        Err(e) => {
            let msg = format!("Could not read the workdir: {}", e);
            return Err(gittools::Error::from_str(&msg));
        }
    };
    for path in iter {
        println!("{:?}", path)
    }
    println!("--------------------------------------------");
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

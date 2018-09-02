extern crate git2;

use git2::{Repository, BranchType};
use std::fs;

fn find_branches(repo: &Repository) -> Result<Vec<String>, git2::Error> {
    let mut result = Vec::new();
    let branches = repo.branches(Some(BranchType::Local))?;
    for maybe_branch in branches {
        let branch = maybe_branch?;
        match branch.0.into_reference().name() {
            Some(b) => result.push(b.to_string()),
            None => {}
        }
    };
    Ok(result)
}

fn checkout_branch(repo: &Repository, ref_name: &str) -> Result<(), git2::Error> {
    let treeish = repo.revparse_single(ref_name)?;
    repo.checkout_tree(&treeish, None)?;
    repo.set_head(ref_name)?;
    Ok(())
}

fn look_at_branches(repo: &Repository, branches: &Vec<String>) -> Result<(), git2::Error> {
    for b in branches {
        println!("Branch: {}", b);
        checkout_branch(&repo, &b)?;
        let workdir = match repo.workdir() {
            None => return Err(git2::Error::from_str("Cannot work on a bare repo")),
            Some(wd) => wd
        };
        let iter = match fs::read_dir(workdir) {
            Ok(val) => val,
            Err(e) => {
                let msg = format!("Could not read the workdir: {}", e);
                return Err(git2::Error::from_str(&msg));
            }
        };
        for path in iter {
            println!("{:?}", path)
        }
        println!("--------------------------------------------");
    }
    Ok(())
}

fn run() -> Result<(), git2::Error> {
    let repo = Repository::open("/home/pal/rust_again/renamee")?;
    let branches = find_branches(&repo)?;
    look_at_branches(&repo, &branches)?;
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

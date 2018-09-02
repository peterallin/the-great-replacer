extern crate git2;

use self::git2::BranchType;
use std::path::Path;

pub use self::git2::{Error, Repository, Signature};

pub fn find_branches(repo: &Repository) -> Result<Vec<String>, git2::Error> {
    let mut result = Vec::new();
    let branches = repo.branches(Some(BranchType::Local))?;
    for maybe_branch in branches {
        let branch = maybe_branch?;
        match branch.0.into_reference().name() {
            Some(b) => result.push(b.to_string()),
            None => {}
        }
    }
    Ok(result)
}

pub fn do_in_branches<F>(
    repo: &Repository,
    branches: &Vec<String>,
    f: F,
) -> Result<(), git2::Error> where
    F: Fn(&str, &Path) -> Result<(), self::Error> {
    for branch in branches {
        checkout_branch(&repo, &branch)?;
        let workdir = match repo.workdir() {
            None => return Err(git2::Error::from_str("Cannot work on a bare repo")),
            Some(wd) => wd,
        };
        f(branch, workdir)?;
    }
    Ok(())
}

pub fn signature<'a>(name: &'a str, email: &'a str) -> Result<Signature<'a>, git2::Error> {
    Signature::now(name, email)
}

fn checkout_branch(repo: &Repository, ref_name: &str) -> Result<(), git2::Error> {
    let treeish = repo.revparse_single(ref_name)?;
    repo.checkout_tree(&treeish, None)?;
    repo.set_head(ref_name)?;
    Ok(())
}

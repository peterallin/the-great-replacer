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


pub fn commit_single_file(repo: &Repository, file_path: &Path, name: &str, email: &str, msg: &str) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_path(file_path)?;
    index.write()?;
    let tree_oid = index.write_tree()?;
    let head = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    let parent_commit = match head.into_commit() {
        Ok(commit) => commit,
        Err(obj) => return Err(git2::Error::from_str(&format!("{} is not a commit", obj.id())))
    };
    let new_tree = repo.find_tree(tree_oid)?;
    let sig = Signature::now(name, email)?;
    repo.commit(Some("HEAD"), &sig, &sig, msg, &new_tree, &[&parent_commit])?;
    Ok(())
}

fn checkout_branch(repo: &Repository, ref_name: &str) -> Result<(), git2::Error> {
    let treeish = repo.revparse_single(ref_name)?;
    repo.checkout_tree(&treeish, None)?;
    repo.set_head(ref_name)?;
    Ok(())
}

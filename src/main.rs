extern crate git2;

use git2::{Repository, BranchType};

fn find_branches(repo_path: &str) -> Result<Vec<String>, git2::Error> {
    let mut result = Vec::new();
    let repo = Repository::open(repo_path)?;
    let branches = repo.branches(Some(BranchType::Local))?;
    for maybe_branch in branches {
        let branch = maybe_branch?;
        match branch.0.into_reference().name() {
            Some(b) => result.push(b.to_string()),
            None => {}
        }
    };
    repo.set_head("refs/heads/old")?;
    Ok(result)
}

fn run() -> Result<(), git2::Error> {
    let branches = find_branches("/home/pal/rust_again/renamee")?;
    for b in branches {
        println!("Branch: {}", b);
    }
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

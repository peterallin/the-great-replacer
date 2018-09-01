extern crate git2;

use git2::{Repository, BranchType};

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

fn run() -> Result<(), git2::Error> {
    let repo = Repository::open("/home/pal/rust_again/renamee")?;
    let branches = find_branches(&repo)?;
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

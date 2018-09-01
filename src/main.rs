extern crate git2;

use git2::{Repository, BranchType};

fn run() -> Result<(), git2::Error> {
    let repo = Repository::open("/home/pal/rust_again/renamee")?;
    let branches = repo.branches(Some(BranchType::Local))?;
    for maybe_branch in branches {
        let branch = maybe_branch?;
        println!("Branch: {:?}", branch.0.into_reference().name())
    };
    repo.set_head("refs/heads/old")?;
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

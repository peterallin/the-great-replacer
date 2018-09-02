extern crate git2;

mod gittools;
use gittools::{find_branches, do_in_branches, Repository};

mod replace;

use std::path::Path;

fn run() -> Result<(), gittools::Error> {
    let change_path = Path::new("snafu");
    let repo = Repository::open("/home/pal/rust_again/renamee")?;
    let branches = find_branches(&repo)?;
    let sig = git2::Signature::now("The Great Replacer", "repl@repl.repl")?;
    let message = "Replaced by the great replacer";

    do_in_branches(&repo, &branches, |_branch, workdir| {
        let change_path_abs = workdir.join(change_path);
        match replace::replace_in_file(change_path_abs.as_path(), "A", "AA") {
            Ok(_) => {}
            Err(e) => return Err(gittools::Error::from_str(&format!("Replacing failed: {}", e)))
        };
        let mut index = repo.index()?;
        index.add_path(change_path)?;
        index.write()?;
        let tree_oid = index.write_tree()?;
        let head = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
        let parent_commit = match head.into_commit() {
            Ok(commit) => commit,
            Err(obj) => return Err(git2::Error::from_str(&format!("{} is not a commit", obj.id())))
        };
        let new_tree = repo.find_tree(tree_oid)?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &new_tree, &[&parent_commit])?;
        Ok(())
    })?;
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

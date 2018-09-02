extern crate git2;

mod gittools;
use gittools::{do_in_branches, find_branches, Repository};

mod replace;

use std::path::Path;

fn replace(
    repo_path: &Path,
    filename: &str,
    name: &str,
    email: &str,
    message: &str,
) -> Result<(), gittools::Error> {
    let change_path = Path::new(filename);
    let repo = Repository::open(repo_path)?;
    let branches = find_branches(&repo)?;
    let sig = gittools::signature(name, email)?;

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
    match replace(
        Path::new("/home/pal/rust_again/renamee"),
        "snafu",
        "The Great Replacer",
        "repl@repl.repl",
        "Replaced by the great replacer",
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

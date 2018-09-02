extern crate git2;

mod gittools;
use gittools::{commit_single_file, do_in_branches, find_branches, Repository};

mod replace;

use std::path::Path;

fn replace(
    repo_path: &Path,
    filename: &str,
    from: &str,
    to: &str,
    name: &str,
    email: &str,
    message: &str,
) -> Result<(), gittools::Error> {
    let change_path = Path::new(filename);
    let repo = Repository::open(repo_path)?;
    let branches = find_branches(&repo)?;

    do_in_branches(&repo, &branches, |_branch, workdir| {
        let change_path_abs = workdir.join(change_path);
        match replace::replace_in_file(change_path_abs.as_path(), from, to) {
            Ok(_) => {}
            Err(e) => return Err(gittools::Error::from_str(&format!("Replacing failed: {}", e)))
        };
        commit_single_file(&repo, &change_path, name, email, message)?;
        Ok(())
    })?;
    Ok(())
}

fn main() {
    match replace(
        Path::new("/home/pal/rust_again/renamee"),
        "snafu",
        "U",
        "UU",
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

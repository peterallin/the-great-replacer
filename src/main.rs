extern crate clap;

mod gittools;
use gittools::{commit_single_file, do_in_branches, find_branches, Repository};

mod replace;

use std::path::Path;
use clap::{App, Arg};

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
    let matches = App::new("the-great-replacer")
        .version("0.1.0")
        .about("Replaces string in all git branches")
        .author("Peter Allin")
        .arg(Arg::with_name("repopath")
            .long("repopath")
            .value_name("PATH")
            .required(true))
        .arg(Arg::with_name("filename")
            .long("filename")
            .value_name("FILENAME")
            .required(true))
        .arg(Arg::with_name("username")
            .long("username")
            .value_name("USERNAME")
            .required(true))
        .arg(Arg::with_name("email")
            .long("email")
            .value_name("EMAIL")
            .required(true))
        .arg(Arg::with_name("from")
            .long("from")
            .value_name("FROM")
            .required(true))
        .arg(Arg::with_name("to")
            .long("to")
            .value_name("TO")
            .required(true))
        .arg(Arg::with_name("message")
            .long("message")
            .value_name("MESSAGE")
            .required(true))
        .get_matches();

    let repo_path = matches.value_of("repopath").unwrap();
    let file_name = matches.value_of("filename").unwrap();
    let user_name = matches.value_of("username").unwrap();
    let email = matches.value_of("email").unwrap();
    let from =  matches.value_of("from").unwrap();
    let to =  matches.value_of("to").unwrap();
    let message =  matches.value_of("message").unwrap();

    match replace(
        Path::new(repo_path),
        file_name,
        from,
        to,
        user_name,
        email,
        message,
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

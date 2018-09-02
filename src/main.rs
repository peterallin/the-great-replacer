mod gittools;
use gittools::{Repository, find_branches, look_at_branches};

fn run() -> Result<(), gittools::Error> {
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

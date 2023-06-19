use anyhow::{Result, Error, Context};
use std::fs;


fn fallible_func(path: String) -> Result<(), Error>{
    // `Context` creates a more helpful errors message when things go wrong
    let content = fs::read_to_string(&path)
        .with_context(|| format!("File not found: {}", path))?;

    // Alternative for
    fs::read_to_string(path)?;
    Ok(())
}

fn main() {
    println!("Testing anyhow - Helpful troubleshoot message.");
    fallible_func(String::from("file_does_not_exists.txt")).unwrap();
}

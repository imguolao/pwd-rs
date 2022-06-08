use std::{env, io};

fn main() -> io::Result<()> {
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}
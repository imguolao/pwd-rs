use std::{env, io};

fn get_current_dir() -> io::Result<String> {
    let path = format!("{:?}", env::current_dir()?);
    Ok(path)
}

fn main() -> io::Result<()> {
    let path = match get_current_dir() {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };

    println!("{}", path);
    
    Ok(())
}
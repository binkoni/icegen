use std::error::Error;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).map_or(env::current_dir()?, |path| PathBuf::from(path));

    for entry in WalkDir::new(path) {
        println!("{}", entry?.path().to_str().ok_or("invalid path")?);
    }
    Ok(())
}

use std::error::Error;
use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;
use icegen::Generator;
struct Test;
impl icegen::Generator for Test {
    fn generate(&self) {
        println!("generate");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let path = args.get(1).map_or(env::current_dir()?, |path| PathBuf::from(path));

    for entry in WalkDir::new(path)
                         .follow_links(true)
                         .into_iter()
                         .filter_entry(|entry| true) {
        println!("{}", entry?.path().to_str().ok_or("Invalid path found during traversal")?);
    }
    let t = Test{};
    t.generate();
    Ok(())
}

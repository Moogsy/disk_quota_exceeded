mod tree;
mod config;

use std::path::PathBuf;

use clap::Parser;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse();

    for path in config.initial_path.iter() {
        let initial_path = PathBuf::from(path);

        match tree::Tree::new(initial_path.clone(), &config) {
            Ok(tree) => {
                tree.display();
            },
            Err(err) => {
                eprintln!(
                    "Couldn't open {}: {err}",
                    initial_path.display(),
                );
            }
            
        }
    }

    Ok(())
}

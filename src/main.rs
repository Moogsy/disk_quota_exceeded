mod tree;

use std::path::PathBuf;

use clap::Parser;

mod config;
mod formatter;
use formatter::Tree;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::parse();
    println!("{:#?}", config);

    let initial_path = PathBuf::from(config.initial_path.clone());
    let tree = Tree::new(initial_path, config);

    println!("{:#?}", tree);


    Ok(())
}

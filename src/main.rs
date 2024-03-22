mod tree;

use clap::Parser;
use tree::Directory;

mod config;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_path = std::env::current_dir()?;

    let mut directory = Directory::new(initial_path);
    directory.build();

    println!("{:#?}", directory);

    let config = config::Config::parse();
    println!("{:#?}", config);


    Ok(())
}

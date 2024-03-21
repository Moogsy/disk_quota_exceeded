mod tree;

use tree::Directory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let initial_path = std::env::current_dir()?;

    let mut directory = Directory::new(initial_path);

    println!("{:#?}", directory);

    directory.build();

    println!("{:#?}", directory);


    Ok(())
}

use clap::Parser;

#[derive(Parser, Debug)]
/// An utility that helps you find what is making 
/// you reach the infamous disk quota exceeded message.
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(default_value = ".")]
    pub initial_path: String,

    #[arg(short, long, default_value_t = false)]
    pub sort_files: bool
}


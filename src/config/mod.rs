use std::ffi::OsString;

use clap::Parser;

#[derive(Parser, Debug)]
/// An utility that helps you find what is making 
/// you reach the infamous disk quota exceeded message.
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(default_value = ".")]
    pub initial_path: Vec<OsString>,

    #[arg(long, default_value = "└──")]
    pub elbow: String, 

    #[arg(long, default_value = "│  ")]
    pub pipe: String,

    #[arg(long, default_value = "├──")]
    pub tee: String,

    #[arg(long, default_value = "   ")]
    pub blank: String
}


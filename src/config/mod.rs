use std::ffi::OsString;

use clap::{Args, Parser};

/// Formatting options
#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Formatting {
    /// The elbow value connecting the last directory
    #[arg(long, group = "format", default_value = "└──")]
    pub elbow: String, 

    /// The character used for pipe
    #[arg(long, group = "format", default_value = "│  ")]
    pub pipe: String,

    /// The prefix for a directory
    #[arg(long, group = "format", default_value = "├──")]
    pub tee: String,

    /// Indentation used to separate directories
    #[arg(long, group = "format", default_value = "   ")]
    pub blank: String,

    /// Whether size units should be converted to a human readable format
    #[arg(long, group = "format", default_value_t = false)]
    pub human_readable: bool,

    /// Whether folders and files should be sorted
    #[arg(long, group = "format", default_value_t = true)]
    pub sort: bool,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Metadata {
    #[arg(long, group = "metadata", default_value_t = true)]
    pub disk_usage: bool
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct Filtering {
    /// Whether entries whose name start with a '.' should be shown
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// Whether empty directories should not be displayed
    #[arg(short, long, default_value_t = false)]
    pub prune: bool,

    /// Only output directories
    #[arg(short, long, default_value_t = false)]
    pub directories_only: bool
}

#[derive(Parser, Debug)]
/// An utility that helps you find what is making 
/// you reach the infamous disk quota exceeded message.
#[command(version, about, long_about = None)]
pub struct Config {
    
    #[command(flatten)]
    pub formatting: Formatting,

    #[command(flatten)]
    pub filtering: Filtering,

    #[command(flatten)]
    pub metadata: Metadata,

    /// Which paths to explore
    #[arg(default_value = ".")]
    pub initial_path: Vec<OsString>,


}


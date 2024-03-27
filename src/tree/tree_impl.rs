use std::io::Write;
use std::{io::stdout, path::PathBuf};

use std::fs;
use crate::{config::Config, tree::Directory};

#[derive(Debug)]
pub struct Tree<'a> {
    root: Directory,
    config: &'a Config
}

impl<'a> Tree<'a> {
    pub fn new(path: PathBuf, config: &'a Config) -> Result<Self, std::io::Error> {
        let metadata = fs::metadata(&path)?; 
        let mut root = Directory::new(path, metadata); 
        root.build(config);
        Ok(Self { root, config })
    }

    pub fn display(&self)
    {
        self.root.display(self.config, String::new(), true);
    }

}

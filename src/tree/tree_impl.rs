use std::path::PathBuf;

use std::fs;
use crate::config::Config;

use super::entry::Entry;

#[derive(Debug)]
pub struct Tree<'a> {
    root: Entry,
    config: &'a Config
}

impl<'a> Tree<'a> {
    pub fn new(path: PathBuf, config: &'a Config) -> Result<Self, std::io::Error> {
        let metadata = fs::metadata(&path)?; 
        let mut root = Entry::new(path, metadata); 
        root.build();
        Ok(Self { root, config })
    }

    pub fn display(&self)
    {
        println!("{:#?}", self.root);
        // self.root.display(self.config, String::new(), true, true);
    }
}

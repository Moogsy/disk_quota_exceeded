use std::path::PathBuf;

use crate::{config::Config, tree::Directory};

#[derive(Debug)]
pub struct Tree {
    root: Directory,
    config: Config
}

impl Tree {
    pub fn new(path: PathBuf, config: Config) -> Self {
        let mut root = Directory::new(path);
        root.build(&config);

        Self { root, config }
    }
}

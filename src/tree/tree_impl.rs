use std::collections::VecDeque;
use std::path::PathBuf;

use super::Directory;

pub struct Tree {
    root: Directory
}

impl Tree {
    pub fn new(path: PathBuf) -> Self {
        let mut root = Directory::new(path);
        Self::build(&mut root);

        Self { root }
    }

    fn build(root: &mut Directory) {
        let mut to_explore = VecDeque::new();
        to_explore.push_back(root);

        while let Some(dir) = to_explore.pop_back() {
        }


    }
}

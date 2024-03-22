use std::io;
use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Entry {
    path: PathBuf,
    metadata: Result<Metadata, io::Error>
}

impl Entry {
    pub fn new(entry: DirEntry, metadata: Result<Metadata, io::Error>) -> Self {
        let path = entry.path();

        Self { path, metadata }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}


impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.path.partial_cmp(&other.path)
    }
}


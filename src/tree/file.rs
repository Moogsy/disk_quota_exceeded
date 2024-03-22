use std::io;
use std::fs::{DirEntry, FileType};

#[derive(Debug)]
pub struct File {
    entry: DirEntry,
    maybe_file_type: Result<FileType, io::Error>,
    maybe_size: Result<u64, io::Error>
}

impl File {
    pub fn new(entry: DirEntry, maybe_file_type: Result<FileType, io::Error>) -> Self {
        let maybe_metadata = entry.metadata();
        let maybe_size = maybe_metadata.map(|meta| meta.len());

        Self { entry, maybe_file_type, maybe_size }
    }
}


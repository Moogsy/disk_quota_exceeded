use std::{fs::{DirEntry, Metadata}, intrinsics::unlikely, io, path::PathBuf};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};



#[derive(Debug)]
pub enum EntryMeta {
    Dir {
        children: Vec<Entry>,
        read_error: Option<io::Error>,
        c_file_count: usize
    },
    File,
    Symlink,
    Uncategorized(io::Error),
}

impl EntryMeta {
    pub fn dir() -> Self {
        Self::Dir { 
            children: Vec::new(), 
            read_error: None, 
            c_file_count: 0 
        }
    }

    pub fn new(metadata: &Metadata) -> Self {
        if metadata.is_file() {
            Self::File
        } else if metadata.is_dir() {
            Self::dir()
        } else if metadata.is_symlink() {
            Self::Symlink
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    path: PathBuf,
    metadata: Metadata,
    c_size: usize,
    entry_meta: EntryMeta
}

impl Entry {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {

        Self {
            path,
            metadata,
            c_size: 0,
            entry_meta: EntryMeta::new(&metadata)
        }
    }

    pub fn build(&mut self) {
    }

    fn build_direct_children(&mut self) {

        if let EntryMeta::Symlink = self.entry_meta {
            if let Ok(canon) = self.path.canonicalize() {
                if let Ok(new_meta) = canon.metadata() {
                    self.path = canon;
                    self.metadata = new_meta;
                    self.entry_meta = EntryMeta::new(&self.metadata);
                }
            }
        }

        let EntryMeta::Dir { 
            children, 
            read_error, 
            c_file_count 
        } = &mut self.entry_meta else {return};
    }
}
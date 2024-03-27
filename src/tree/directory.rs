use std::borrow::Cow;
use std::io::Write;
use std::fs::{self, DirEntry, Metadata};
use std::{io, u64};
use std::path::PathBuf;

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::config::Config;

use super::Entry;


/// Represents a directory in the file system
#[derive(Debug)]
pub struct Directory {
    /// The path uses to acess this directory
    path: PathBuf,

    /// Metadata (size, permissions, ...) associated to it
    metadata: Metadata,

    /// Any error that occured when trying to read 
    /// the contents of this directory
    can_read_dir_error: Option<io::Error>,

    /// Any errors that occured while reading this 
    /// directory's contents
    read_dir_errors: Vec<io::Error>,

    /// All of this directory's contents that were
    /// detected as directories
    subdirs: Vec<Directory>,

    /// Any other entry that weren't detected as 
    /// a directory
    children: Vec<Entry> ,

    /// The amount of files that this directory and it's 
    /// subdirs contains
    /// In an NTFS file system, it can't go over
    /// the u32::MAX
    cumulative_file_count: u64,

    /// Total size of files contained in this directory 
    /// and it's subdirs
    cumulative_size: u128
}

impl Directory {
    pub fn new(path: PathBuf, metadata: Metadata) -> Self {
        Self {
            path,
            metadata,

            can_read_dir_error: None,
            read_dir_errors: Vec::new(),

            children: Vec::new(),
            subdirs: Vec::new(),

            cumulative_file_count: 0,
            cumulative_size: 0,
        }
    }

    pub fn display(
        &self, 
        config: &Config,
        header: String,
        is_last: bool,
    ) {
        let anchor = if is_last {"└──"} else {"├──"};

        println!("{header}{}{}", anchor, self.path.iter().last().unwrap().to_str().unwrap());

        for (index, subdir) in self.subdirs.iter().enumerate() {
            let is_last = index + 1 == self.subdirs.len();

            let mut new_header = header.clone();
            new_header += "│  ";
            
            subdir.display(config, new_header, is_last);
        }

    }

    pub fn build(&mut self, config: &Config) {
        self.build_direct_children();

        let (subleaf_files_count, subleaf_size): (u64, u128) = self.subdirs
            .par_iter_mut()
            .map(|subdir| {
                subdir.build(config);
                (subdir.cumulative_file_count, subdir.cumulative_size)
            })
            .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        self.cumulative_file_count = (self.children.len() as u64) + subleaf_files_count;

        let children_size: u128 = self.children
            .par_iter()
            .map(|child| {
                match child.metadata() {
                    Ok(metadata) => metadata.len() as u128,
                    Err(_) => 0
                }
            })
            .sum();

        self.cumulative_size = subleaf_size + children_size + (self.metadata.len() as u128);
    }

    fn build_direct_children(&mut self) {
        match fs::read_dir(&self.path) {
            Err(err) => {
                self.can_read_dir_error = Some(err);
            }
            Ok(entries) => {
                self.handle_entries(entries);
            }
        };
    }

    fn handle_entries<T>(&mut self, entries: T) 
    where 
        T: Iterator<Item = Result<DirEntry, io::Error>>
    {
        for maybe_entry in entries {
            match maybe_entry {
                Ok(entry) => self.handle_entry(entry),
                Err(e) => self.read_dir_errors.push(e),
            }
        }
    }
    fn handle_entry(&mut self, entry: DirEntry){
        match entry.metadata() {
            Ok(metadata) if metadata.is_dir() => {
                let subdir = Directory::new(entry.path(), metadata);
                self.subdirs.push(subdir);
            },
            file_or_unknown => {
                let child = Entry::new(entry, file_or_unknown);
                self.children.push(child);
            }
        }
    }

}



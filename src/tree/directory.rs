use std::fs::{self, DirEntry};
use std::io;
use std::path::PathBuf;

// use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::config::Config;

use super::Entry;


#[derive(Debug)]
pub struct Directory {
    path: PathBuf,

    can_read_dir_error: Option<io::Error>,
    read_dir_errors: Vec<io::Error>,

    children: Vec<Entry> ,
    subdirs: Vec<Directory>,

    contained_files_count: usize
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,

            can_read_dir_error: None,
            read_dir_errors: Vec::new(),

            children: Vec::new(),
            subdirs: Vec::new(),

            contained_files_count: 0
        }
    }

    pub fn build(&mut self, config: &Config) -> usize {
        self.build_direct_children();

        let subleaf_count: usize = self.subdirs
            .par_iter_mut()
            .map(|subdir| subdir.build(config))
            .sum();

        self.contained_files_count = self.children.len() + subleaf_count;

        self.process_config(config);

        self.contained_files_count
    }

    fn process_config(&mut self, config: &Config) {
        if config.sort_dirs {
            todo!()
        }

        if config.sort_files {
            todo!()
        }
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
                let subdir = Directory::new(entry.path());
                self.subdirs.push(subdir);
            },
            file_or_unknown => {
                let child = Entry::new(entry, file_or_unknown);
                self.children.push(child);
            }
        }
    }

}



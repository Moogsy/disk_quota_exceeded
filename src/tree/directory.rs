use std::fs::{self, DirEntry, FileType};
use std::io;
use std::path::PathBuf;
use std::collections::VecDeque;

use super::File;


#[derive(Debug)]
pub struct Directory {
    path: PathBuf,

    can_read_dir_error: Option<io::Error>,
    read_dir_errors: Vec<io::Error>,

    children: Vec<File> ,
    subdirs: Vec<Directory>,

    total_contained_files_count: usize
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,

            can_read_dir_error: None,
            read_dir_errors: Vec::new(),

            children: Vec::new(),
            subdirs: Vec::new(),

            total_contained_files_count: 0
        }
    }

    pub fn build(&mut self) {
        self.build_direct_children();
        self.total_contained_files_count = self.children.len();

        for subdir in self.subdirs.iter_mut() {
            subdir.build_direct_children();

            self.total_contained_files_count += subdir.children.len();
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

    fn make_child(&mut self, entry: DirEntry, file_type: Result<FileType, io::Error>) {
        let child = File::new(entry, file_type);
        self.children.push(child);
    }

    fn handle_typed_entry(&mut self, entry: DirEntry, file_type: FileType) {

        let path = entry.path();

        if file_type.is_dir() {
            let subdir = Directory::new(path);
            self.subdirs.push(subdir);
        } 
        else {
            self.make_child(entry, Ok(file_type));
        }
    }

    fn handle_entry(&mut self, entry: DirEntry){
        match entry.file_type() {
            Ok(file_type) => {
                self.handle_typed_entry(entry, file_type);
            },
            Err(error) => {
                self.make_child(entry, Err(error));
            }
        }
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

}



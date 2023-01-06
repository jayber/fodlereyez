use std::{env, fs};
use std::error::Error;
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::path::PathBuf;
use folder_size::analysis::*;

fn main() -> Result<(), Box<dyn Error>> {
    run(env::current_dir()?, &RealFileOperations)
}

pub struct RealFileOperations;

impl FileSystemProxy for RealFileOperations {
    fn read_dir(&self, directory: &PathBuf) -> std::io::Result<Box<dyn ReadDirProxy<Item=Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>> {
        Ok(Box::new(RealReadDir::new(fs::read_dir(directory)?)))
    }
    fn metadata(&self, path: &PathBuf) -> std::io::Result<Box<dyn MetadataProxy>> {
        Ok(Box::new(RealMetadataProxy{metadata: fs::metadata(path)?}))
    }
}

pub struct RealReadDir {
    read_dir: ReadDir
}
impl RealReadDir {
    fn new(read_dir: ReadDir) -> RealReadDir {
        Self { read_dir }
    }
}

impl Iterator for RealReadDir {
    type Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Ok(Box::new(RealDirPathEntry{ fs_dir_path: self.read_dir.next()?.unwrap()})))
    }
}

impl ReadDirProxy for RealReadDir {}

struct RealDirPathEntry {
    fs_dir_path: DirEntry
}

impl DirPathEntryProxy for RealDirPathEntry {
    fn path(&self) -> PathBuf {
        self.fs_dir_path.path()
    }
    fn file_type(&self) -> std::io::Result<Box<dyn FileTypeProxy>> {
        Ok(Box::new(RealFileTypeProxy{file_type: self.fs_dir_path.file_type()?}))
    }
}

struct RealFileTypeProxy {
    file_type: FileType
}

impl FileTypeProxy for RealFileTypeProxy {
    fn is_dir(&self) -> bool {
        self.file_type.is_dir()
    }
}

struct RealMetadataProxy {
    metadata: Metadata,
}

impl MetadataProxy for RealMetadataProxy {
    fn len(&self) -> u64 {
        self.metadata.len()
    }
}
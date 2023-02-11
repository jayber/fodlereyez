use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::{DirEntry, FileType, Metadata, ReadDir};
use std::path::PathBuf;
use std::{fs, mem};

use crate::file_analysis::proxies::*;

pub(crate) struct RealFileOperations;

impl FileSystemProxy for RealFileOperations {
    fn read_dir(
        &self, directory: &PathBuf
    ) -> Result<Box<dyn ReadDirProxy<Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>, Box<dyn Error>> {
        let read_dir = fs::read_dir(directory).map_err(|e| FSProxyError { path: directory.clone(), source: e })?;
        Ok(Box::new(RealReadDir::new(read_dir, directory.clone())))
    }
    fn metadata(&self, path: &PathBuf) -> Result<Box<dyn MetadataProxy>, Box<dyn Error>> {
        Ok(Box::new(RealMetadataProxy {
            metadata: fs::metadata(path).map_err(|e| FSProxyError { path: path.clone(), source: e })?
        }))
    }
}

pub(crate) struct FSProxyError {
    path: PathBuf,
    source: std::io::Error
}

impl Debug for FSProxyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.write_message(f) }
}

impl FSProxyError {
    fn write_message(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "path {}, caused by {}", self.path.to_str().unwrap_or("unknown"), self.source)
    }
}

impl Display for FSProxyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { self.write_message(f) }
}

impl Error for FSProxyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> { Some(&self.source) }
}

pub(crate) struct RealReadDir {
    read_dir: ReadDir,
    path: PathBuf
}

impl RealReadDir {
    fn new(read_dir: ReadDir, directory: PathBuf) -> RealReadDir { Self { read_dir, path: directory } }
}

impl Iterator for RealReadDir {
    type Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Ok(Box::new(RealDirPathEntry { fs_dir_path: self.read_dir.next()?.unwrap() })))
    }
}

impl ReadDirProxy for RealReadDir {
    fn path(&mut self) -> PathBuf {
        let path = mem::replace(&mut self.path, PathBuf::new());
        path
    }
}

struct RealDirPathEntry {
    fs_dir_path: DirEntry
}

impl DirPathEntryProxy for RealDirPathEntry {
    fn path(&self) -> PathBuf { self.fs_dir_path.path() }
    fn file_type(&self) -> std::io::Result<Box<dyn FileTypeProxy>> {
        Ok(Box::new(RealFileTypeProxy { file_type: self.fs_dir_path.file_type()? }))
    }
}

struct RealFileTypeProxy {
    file_type: FileType
}

impl FileTypeProxy for RealFileTypeProxy {
    fn is_dir(&self) -> bool { self.file_type.is_dir() }
}

struct RealMetadataProxy {
    metadata: Metadata
}

impl MetadataProxy for RealMetadataProxy {
    fn len(&self) -> u64 { self.metadata.len() }
}

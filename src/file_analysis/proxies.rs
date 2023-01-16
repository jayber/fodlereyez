#[cfg(test)]
use mockall::automock;

use std::error::Error;
use std::path::PathBuf;

#[cfg_attr(test, automock)]
pub trait FileSystemProxy {
    fn read_dir(
        &self, directory: &PathBuf
    ) -> Result<
        Box<dyn ReadDirProxy<Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>,
        Box<dyn Error>
    >;
    fn metadata(&self, path: &PathBuf) -> Result<Box<dyn MetadataProxy>, Box<dyn Error>>;
}

pub trait ReadDirProxy: Iterator {}

#[cfg_attr(test, automock)]
pub trait DirPathEntryProxy {
    fn path(&self) -> PathBuf;
    fn file_type(&self) -> std::io::Result<Box<dyn FileTypeProxy>>;
}

#[cfg_attr(test, automock)]
pub trait FileTypeProxy {
    fn is_dir(&self) -> bool;
}

#[cfg_attr(test, automock)]
pub trait MetadataProxy {
    fn len(&self) -> u64;
}

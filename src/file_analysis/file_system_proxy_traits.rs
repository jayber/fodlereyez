use std::error::Error;
use std::path::PathBuf;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub(crate) trait FileSystemProxy {
    fn read_dir(
        &self, directory: &PathBuf,
    ) -> Result<Box<dyn ReadDirProxy<Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>, Box<dyn Error>>;
    fn metadata(&self, path: &PathBuf) -> Result<Box<dyn MetadataProxy>, Box<dyn Error>>;
}

pub(crate) trait ReadDirProxy: Iterator {
    fn path(&mut self) -> PathBuf;
}

#[cfg_attr(test, automock)]
pub(crate) trait DirPathEntryProxy {
    fn path(&self) -> PathBuf;
    fn file_type(&self) -> std::io::Result<Box<dyn FileTypeProxy>>;
    fn metadata(&self) -> Result<Box<dyn MetadataProxy>, Box<dyn Error>>;
}

#[cfg_attr(test, automock)]
pub(crate) trait FileTypeProxy {
    fn is_dir(&self) -> bool;
}

#[cfg_attr(test, automock)]
pub(crate) trait MetadataProxy {
    fn len(&self) -> u64;
    fn file_attributes(&self) -> u32;
}

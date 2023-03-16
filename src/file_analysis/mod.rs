use std::path::PathBuf;

use file_system_proxy_traits::FileSystemProxy;
use file_types::Byteable;

use crate::file_analysis::file_system_proxy_traits::MetadataProxy;
use crate::file_analysis::file_types::DirectoryEntry;

pub(crate) mod file_system_proxy_traits;
pub(crate) mod file_types;

pub(crate) fn read_fs(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryEntry {
    populate_tree(file_operations, current_dir, true)
}

fn populate_tree(file_operations: &impl FileSystemProxy, current_dir: PathBuf, is_root: bool) -> DirectoryEntry {
    if let Ok(directory_entries) = file_operations.read_dir(&current_dir) {
        let mut len = 0_u64;
        let mut entries = vec![];
        for entry in directory_entries {
            let entry = entry.expect("error in getting entry");
            let entry_path = entry.path();
            if entry.file_type().expect("error getting file type").is_dir() {
                let child = populate_tree(file_operations, entry_path, false);
                len += child.len().0;
                entries.push(child);
            } else if let Ok(metadata) = file_operations.metadata(&entry_path) {
                len += metadata.len();
                let len = Byteable(metadata.len());
                let hidden = is_hidden(file_operations, &current_dir);
                entries.push(DirectoryEntry::new_file(len, entry_path, hidden));
            }
        }
        let hidden = is_hidden(file_operations, &current_dir);
        DirectoryEntry::new_folder(Byteable(len), current_dir, hidden, entries, is_root)
    } else {
        DirectoryEntry::new_folder(Byteable(0), current_dir, false, vec![], is_root)
    }
}

fn is_hidden(file_operations: &impl FileSystemProxy, current_dir: &PathBuf) -> bool {
    #[cfg(target_os = "windows")]
    return file_operations.metadata(&current_dir).map(|m| (m.file_attributes() & 0b_10) == 0b_10).unwrap_or(true);

    #[cfg(not(target_os = "windows"))]
    return current_dir.file_name().and_then(|name| name.to_str()).map(|name| name.starts_with(".")).unwrap_or(false);
}

#[cfg(test)]
mod mock_utils;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::file_analysis::{mock_utils, read_fs};

    #[test]
    fn test_run_with_1_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(0, 1)?;
        let entry = read_fs(dir, &mock_file_operations);
        assert_eq!(entry.len().0, 1024 * 1024_u64);
        if let Some(entries) = entry.entries() {
            let mut iter = entries.iter();
            let file = iter.next();
            assert_eq!(file.is_some(), true);
            let file = file.unwrap();
            assert_eq!(file.is_dir(), false);
            assert_eq!(file.len().0, 1024 * 1024_u64);
            assert_eq!(iter.next().is_none(), true);
            Ok(())
        } else {
            panic!("None should be some")
        }
    }

    #[test]
    fn test_run_with_1_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 0)?;
        let entry = read_fs(dir, &mock_file_operations);
        assert_eq!(entry.len().0, 0_u64);
        if let Some(entries) = entry.entries() {
            let mut iter = entries.iter();
            let child = iter.next();
            assert_eq!(child.is_some(), true);
            let child = child.unwrap();
            assert_eq!(child.is_dir(), true);
            assert_eq!(child.name(), "test\\");
            assert_eq!(child.len().0, 0_u64);
            assert_eq!(iter.next().is_none(), true);
            Ok(())
        } else {
            panic!("None should be some")
        }
    }

    #[test]
    fn test_run_with_2_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 0)?;
        let entry = read_fs(dir, &mock_file_operations);
        assert_eq!(entry.len().0, 0_u64);
        if let Some(entries) = entry.entries() {
            let mut iter = entries.iter();
            let child = iter.next();
            assert_eq!(child.is_some(), true);
            let entry = child.unwrap();
            assert_eq!(entry.name(), "test\\");
            assert_eq!(entry.is_dir(), true);

            let child = iter.next();
            assert_eq!(child.is_some(), true);
            let entry1 = child.unwrap();
            assert_eq!(entry1.name(), "test\\");
            assert_eq!(entry1.is_dir(), true);

            assert_eq!(iter.next().is_some(), false);
            Ok(())
        } else {
            panic!("None should be some")
        }
    }

    #[test]
    fn test_run_with_1_directory_and_1_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 1)?;
        let entry = read_fs(dir, &mock_file_operations);
        assert_eq!(entry.len().0, 1024 * 1024_u64);
        if let Some(entries) = entry.entries() {
            let mut iter = entries.iter();

            let file = iter.next();
            assert_eq!(file.is_some(), true);
            let entry = file.unwrap();
            assert_eq!(entry.is_dir(), false);
            assert_eq!(entry.len().0, 1024 * 1024_u64);

            let rollup = iter.next();
            assert_eq!(rollup.is_some(), true);
            let child = rollup.unwrap();
            assert_eq!(child.name(), "test\\");
            assert_eq!(child.is_dir(), true);
            assert_eq!(child.len().0, 0_u64);
            Ok(())
        } else {
            panic!("None should be some")
        }
    }

    #[test]
    fn test_run_with_2_directory_and_2_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 2)?;
        let entry = read_fs(dir, &mock_file_operations);
        assert_eq!(entry.len().0, 1024 * 1024_u64 * 2_u64);
        if let Some(entries) = entry.entries() {
            let mut iter = entries.iter();
            let file = iter.next();
            assert_eq!(file.is_some(), true);
            let entry = file.unwrap();
            assert_eq!(entry.is_dir(), false);
            assert_eq!(entry.len().0, 1024 * 1024_u64);
            let file = iter.next();
            assert_eq!(file.is_some(), true);
            let entry = file.unwrap();
            assert_eq!(entry.is_dir(), false);
            assert_eq!(entry.len().0, 1024 * 1024_u64);

            let child = iter.next();
            assert_eq!(child.is_some(), true);
            let child = child.unwrap();
            assert_eq!(child.is_dir(), true);
            assert_eq!(child.len().0, 0_u64);
            assert_eq!(child.name(), "test\\");

            let child = iter.next();
            assert_eq!(child.is_some(), true);
            let child = child.unwrap();
            assert_eq!(child.is_dir(), true);
            assert_eq!(child.len().0, 0_u64);
            assert_eq!(child.name(), "test\\");

            let child = iter.next();
            assert_eq!(child.is_some(), false);

            Ok(())
        } else {
            panic!("None should be some")
        }
    }
}

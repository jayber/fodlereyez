use std::error::Error;
use std::path::PathBuf;

use file_system_proxy_traits::{DirPathEntryProxy, FileSystemProxy, ReadDirProxy};
use file_types::{Byteable, DirectoryTree};

pub(crate) mod file_system_proxy_traits;
pub(crate) mod file_types;

pub(crate) fn read_fs(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    calc_directory_tree(String::from(current_dir.to_str().unwrap()), &current_dir, file_operations)
}

fn calc_directory_tree(current_dir: String, path: &PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    file_operations.read_dir(path).map_or_else(
        |e| {
            eprintln!("error in read_dir: {}", e);
            DirectoryTree::new(current_dir.clone())
        },
        |read_dir| {
            let tree = DirectoryTree::new(current_dir.clone());
            populate_tree(file_operations, read_dir, tree)
        }
    )
}

fn populate_tree(
    file_operations: &impl FileSystemProxy,
    mut read_dir: Box<dyn ReadDirProxy<Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>,
    mut tree: DirectoryTree
) -> DirectoryTree {
    let path_buf = read_dir.path();
    for entry in read_dir {
        let entry = entry.expect("error in getting entry");
        match entry.file_type().expect("error getting file type").is_dir() {
            true => {
                let dir = entry.path();
                let child = calc_directory_tree(get_file_name(&dir), &dir, file_operations);
                tree.len.val += child.len.val;
                tree.add_directory(child, dir.clone());
            }
            false => match file_operations.metadata(&entry.path()) {
                Ok(metadata) => {
                    tree.len.val += metadata.len();
                    tree.add_file(get_file_name(&entry.path()), Byteable { val: metadata.len() }, PathBuf::new())
                }
                Err(e) => {
                    eprintln!("error in metadata: {}", e);
                }
            }
        }
    }
    tree.rollup(path_buf);
    tree
}

fn get_file_name(buf: &PathBuf) -> String {
    let component = buf.file_name().unwrap();
    String::from(component.to_str().unwrap())
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
        let tree = read_fs(dir, &mock_file_operations);
        assert_eq!(tree.name, "current");
        assert_eq!(tree.len.val, 1024 * 1024_u64);
        let mut iter = tree.entries.into_iter();
        let file = iter.next();
        assert_eq!(file.is_some(), true);
        let file = file.unwrap();
        assert_eq!(file.is_dir(), false);
        assert_eq!(file.len().val, 1024 * 1024_u64);
        assert_eq!(iter.next().is_none(), true);
        Ok(())
    }

    #[test]
    fn test_run_with_1_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 0)?;
        let tree = read_fs(dir, &mock_file_operations);
        assert_eq!(tree.name, "current");
        assert_eq!(tree.len.val, 0_u64);
        let mut iter = tree.entries.into_iter();
        let child = iter.next();
        assert_eq!(child.is_some(), true);
        let child = child.unwrap();
        assert_eq!(child.is_dir(), true);
        assert_eq!(child.name(), "test\\");
        assert_eq!(child.len().val, 0_u64);
        assert_eq!(iter.next().is_none(), true);
        Ok(())
    }

    #[test]
    fn test_run_with_2_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 0)?;
        let tree = read_fs(dir, &mock_file_operations);
        assert_eq!(tree.len.val, 0_u64);
        let mut iter = tree.entries.into_iter();
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
    }

    #[test]
    fn test_run_with_1_directory_and_1_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 1)?;
        let tree = read_fs(dir, &mock_file_operations);
        assert_eq!(tree.name, "current");
        assert_eq!(tree.len.val, 1024 * 1024_u64);
        let mut iter = tree.entries.into_iter();

        let file = iter.next();
        assert_eq!(file.is_some(), true);
        let entry = file.unwrap();
        assert_eq!(entry.is_dir(), false);
        assert_eq!(entry.len().val, 1024 * 1024_u64);

        let rollup = iter.next();
        assert_eq!(rollup.is_some(), true);
        let child = rollup.unwrap();
        assert_eq!(child.name(), "test\\");
        assert_eq!(child.is_dir(), true);
        assert_eq!(child.len().val, 0_u64);
        Ok(())
    }

    #[test]
    fn test_run_with_2_directory_and_2_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 2)?;
        let tree = read_fs(dir, &mock_file_operations);
        assert_eq!(tree.len.val, 1024 * 1024_u64 * 2_u64);
        assert_eq!(tree.name, "current");
        let mut iter = tree.entries.into_iter();

        let file = iter.next();
        assert_eq!(file.is_some(), true);
        let entry = file.unwrap();
        assert_eq!(entry.is_dir(), false);
        assert_eq!(entry.len().val, 1024 * 1024_u64);
        let file = iter.next();
        assert_eq!(file.is_some(), true);
        let entry = file.unwrap();
        assert_eq!(entry.is_dir(), false);
        assert_eq!(entry.len().val, 1024 * 1024_u64);

        let child = iter.next();
        assert_eq!(child.is_some(), true);
        let child = child.unwrap();
        assert_eq!(child.is_dir(), true);
        assert_eq!(child.len().val, 0_u64);
        assert_eq!(child.name(), "test\\");

        let child = iter.next();
        assert_eq!(child.is_some(), true);
        let child = child.unwrap();
        assert_eq!(child.is_dir(), true);
        assert_eq!(child.len().val, 0_u64);
        assert_eq!(child.name(), "test\\");

        let child = iter.next();
        assert_eq!(child.is_some(), false);

        Ok(())
    }
}

use std::error::Error;
use std::path::PathBuf;

use file_objects::{Byteable, DirectoryTree, FileInfo};
use proxies::{DirPathEntryProxy, FileSystemProxy, ReadDirProxy};

pub(crate) mod file_objects;
pub(crate) mod proxies;

pub fn run(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    let mut root = calc_directory_tree(current_dir, file_operations);
    root.children.sort_by(|a, b| b.len.val.partial_cmp(&a.len.val).unwrap());
    root
}

fn calc_directory_tree(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    file_operations.read_dir(&current_dir).map_or_else(
        |e| {
            eprintln!("error in read_dir: {}", e);
            DirectoryTree { name: current_dir.clone(), children: vec![], len: Byteable { val: 0_u64 }, files: vec![] }
        },
        |read_dir| {
            let tree = DirectoryTree {
                name: current_dir.clone(),
                children: vec![],
                len: Byteable { val: 0_u64 },
                files: vec![]
            };
            populate_tree(file_operations, read_dir, tree)
        }
    )
}

fn populate_tree<'a>(
    file_operations: &impl FileSystemProxy,
    read_dir: Box<dyn ReadDirProxy<Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>,
    mut tree: DirectoryTree
) -> DirectoryTree {
    for entry in read_dir {
        let entry = entry.expect("error in getting entry");
        match entry.file_type().expect("error getting file type").is_dir() {
            true => {
                let child = calc_directory_tree(entry.path(), file_operations);
                tree.len.val += child.len.val;
                tree.children.push(child);
            }
            false => match file_operations.metadata(&entry.path()) {
                Ok(metadata) => {
                    tree.len.val += metadata.len();
                    tree.files.push(FileInfo { name: get_file_name(entry), len: Byteable { val: metadata.len() } })
                }
                Err(e) => {
                    eprintln!("error in metadata: {}", e);
                }
            }
        }
    }
    tree
}

fn get_file_name<'a>(entry: Box<dyn DirPathEntryProxy>) -> String {
    entry.path().components().last().unwrap().as_os_str().to_str().unwrap().to_string()
}

#[cfg(test)]
mod mock_utils;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::file_analysis::{mock_utils, run};

    #[test]
    fn test_run_with_1_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(0, 1)?;
        let tree = run(dir, &mock_file_operations);
        assert_eq!(tree.children.len(), 0);
        assert_eq!(tree.files.len(), 1);
        assert_eq!(tree.len.val, 10_u64);
        Ok(())
    }

    #[test]
    fn test_run_with_1_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 0)?;
        let tree = run(dir, &mock_file_operations);
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.files.len(), 0);
        assert_eq!(tree.len.val, 0_u64);
        Ok(())
    }

    #[test]
    fn test_run_with_2_directory() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 0)?;
        let tree = run(dir, &mock_file_operations);
        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.files.len(), 0);
        assert_eq!(tree.len.val, 0_u64);
        Ok(())
    }

    #[test]
    fn test_run_with_1_directory_and_1_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(1, 1)?;
        let tree = run(dir, &mock_file_operations);
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.files.len(), 1);
        assert_eq!(tree.len.val, 10_u64);
        Ok(())
    }

    #[test]
    fn test_run_with_2_directory_and_2_file() -> Result<(), Box<dyn Error>> {
        let (dir, mock_file_operations) = mock_utils::set_expect(2, 2)?;
        let tree = run(dir, &mock_file_operations);
        assert_eq!(tree.children.len(), 2);
        assert_eq!(tree.files.len(), 2);
        assert_eq!(tree.len.val, 20_u64);
        Ok(())
    }
}

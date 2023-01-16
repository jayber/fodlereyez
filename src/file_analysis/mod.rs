use std::path::PathBuf;
use file_objects::{DirectoryTree, FileInfo};

use proxies::FileSystemProxy;

use crate::file_analysis::file_objects::Byteable;

pub(crate) mod file_objects;
pub(crate) mod proxies;

pub fn run(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    let mut root = calc_directory_tree(current_dir, file_operations);
    root.children
        .sort_by(|a, b| b.len.val.partial_cmp(&a.len.val).unwrap());
    root
}

fn calc_directory_tree(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> DirectoryTree {
    let mut total = 0u64;
    let mut sub_directories: Vec<DirectoryTree> = vec![];
    let mut files: Vec<FileInfo> = vec![];
    let read_dir = file_operations.read_dir(&current_dir);
    match read_dir {
        Ok(read_dir) => {
            for entry in read_dir {
                let entry = entry.expect("error in getting entry");
                let path = entry.path();
                let type_of_file = entry.file_type().expect("error getting file type");

                let is_dir = type_of_file.is_dir();
                if is_dir {
                    let child = calc_directory_tree(path, file_operations);

                    total += child.len.val;
                    sub_directories.push(child);
                } else {
                    match file_operations.metadata(&path) {
                        Ok(metadata) => {
                            total += metadata.len();
                            files.push(FileInfo { name: String::from(path.components().last().unwrap().as_os_str().to_str().unwrap()), len: Byteable { val: metadata.len() } })
                        }
                        Err(e) => {
                            eprintln!("error in metadata: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("error in read_dir: {}", e);
        }
    }

    DirectoryTree {
        name: current_dir.clone(),
        children: sub_directories,
        len: Byteable { val: total },
        files,
    }
}

#[cfg(test)]
mod mock_utils;

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::file_analysis::mock_utils;

    #[test]
    fn test_run_with_1_file() -> Result<(), Box<dyn Error>> {
        mock_utils::set_expect(0, 1)
    }

    #[test]
    fn test_run_with_1_directory() -> Result<(), Box<dyn Error>> {
        mock_utils::set_expect(1, 0)
    }

    #[test]
    fn test_run_with_2_directory() -> Result<(), Box<dyn Error>> {
        mock_utils::set_expect(2, 0)
    }

    #[test]
    fn test_run_with_1_directory_and_1_file() -> Result<(), Box<dyn Error>> {
        mock_utils::set_expect(1, 1)
    }

    #[test]
    fn test_run_with_2_directory_and_2_file() -> Result<(), Box<dyn Error>> {
        mock_utils::set_expect(2, 2)
    }
}

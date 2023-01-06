pub mod analysis {
    use std::error::Error;
    use std::fmt;
    use std::path::PathBuf;

    #[cfg(test)]
    use mockall::*;

    pub fn run(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> Result<(), Box<dyn Error>> {
        let mut root = calc_length(current_dir, file_operations)?;
        root.children.sort_by(|a, b| b.len.partial_cmp(&a.len).unwrap());
        println!("This directory {}", root);
        Ok(())
    }

    fn calc_length(current_dir: PathBuf, file_operations: &impl FileSystemProxy) -> Result<DirectoryTree, Box<dyn Error>> {
        let mut total = 0u64;
        let mut sub_directories: Vec<DirectoryTree> = vec![];
        for entry in file_operations.read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();
            let is_dir = entry.file_type().unwrap().is_dir();
            if is_dir {
                let child = calc_length(path, file_operations)?;
                total += child.len;
                sub_directories.push(child);
            } else {
                let metadata = file_operations.metadata(&path)?;
                total += metadata.len();
            }
        }
        Ok(DirectoryTree { name: current_dir.clone(), children: sub_directories, len: total })
    }

    #[derive(Debug)]
    struct DirectoryTree {
        name: PathBuf,
        children: Vec<DirectoryTree>,
        len: u64,
    }

    impl fmt::Display for DirectoryTree {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "{}: {}, sub-directories:", self.name.to_str().unwrap(), self.len)?;
            for child in &self.children {
                writeln!(f, "    {}: {}, sub-directories: {:?}", child.name.to_str().unwrap(), child.len, child.children)?;
            }
            Ok(())
        }
    }

    #[cfg_attr(test, automock)]
    pub trait FileSystemProxy {
        fn read_dir(&self, directory: &PathBuf) -> std::io::Result<Box<dyn ReadDirProxy<Item=Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>>>;
        fn metadata(&self, path: &PathBuf) -> std::io::Result<Box<dyn MetadataProxy>>;
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


    #[cfg(test)]
    mod tests {
        use std::error::Error;
        use std::path::PathBuf;

        use super::*;

        mock! {
            MyReadDirProxy {}
            impl ReadDirProxy for MyReadDirProxy {}
            impl Iterator for MyReadDirProxy {
                type Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>;
                fn next(&mut self) -> Option<Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>;
            }
        }

        #[test]
        fn test_run_with_1_file() -> Result<(), Box<dyn Error>> {
            set_expect(0, 1)
        }

        #[test]
        fn test_run_with_1_directory() -> Result<(), Box<dyn Error>> {
            set_expect(1, 0)
        }

        #[test]
        fn test_run_with_2_directory() -> Result<(), Box<dyn Error>> { set_expect(2, 0) }

        #[test]
        fn test_run_with_1_directory_and_1_file() -> Result<(), Box<dyn Error>> {
            set_expect(1, 1)
        }

        #[test]
        fn test_run_with_2_directory_and_2_file() -> Result<(), Box<dyn Error>> { set_expect(2, 2) }

        fn set_expect(num_directories: usize, num_files: usize) -> Result<(), Box<dyn Error>> {
            let mut mock_file_operations = MockFileSystemProxy::new();
            let dir = PathBuf::new();
            // let buf = dir.clone();
            let mut seq_read_dir = Sequence::new();
            expect_read_dir(num_directories, num_files, &mut mock_file_operations, dir.clone(), &mut seq_read_dir);
            if num_files > 0 {
                mock_file_operations.expect_metadata().times(num_files).returning(|_| {
                    let mut metadata = MockMetadataProxy::new();
                    metadata.expect_len().return_const(10u64);
                    Ok(Box::new(metadata))
                });
            }

            run(dir, &mock_file_operations)?;
            Ok(())
        }

        fn expect_read_dir(num_directories: usize, num_files: usize, mock_file_operations: &mut MockFileSystemProxy, buf: PathBuf, seq_read_dir: &mut Sequence) {
            let buf2 = buf.clone();
            mock_file_operations.expect_read_dir().withf(move |path| *path == buf).times(1).in_sequence(seq_read_dir).returning(move |_dir| {
                let mut mock_read_dir = MockMyReadDirProxy::new();
                let mut seq = Sequence::new();
                for _i in 0..num_directories {
                    expect_read_dir_next(true, &mut mock_read_dir, &mut seq);
                }
                for _j in 0..num_files {
                    expect_read_dir_next(false, &mut mock_read_dir, &mut seq);
                }
                mock_read_dir.expect_next().times(1)
                    .in_sequence(&mut seq)
                    .returning(|| None);
                Ok(Box::new(mock_read_dir))
            });

            for _i in 0..num_directories {
                expect_read_dir(0, 0, mock_file_operations, buf2.clone(), seq_read_dir);
            }
        }

        fn expect_read_dir_next(is_dir: bool, mock_read_dir: &mut MockMyReadDirProxy, seq: &mut Sequence) {
            mock_read_dir.expect_next().times(1)
                .in_sequence(seq)
                .returning(move || {
                    let mut entry = MockDirPathEntryProxy::new();
                    entry.expect_path().returning(|| PathBuf::new());
                    entry.expect_file_type().returning(move || {
                        let mut file_type = MockFileTypeProxy::new();
                        file_type.expect_is_dir().times(1).return_const(is_dir);
                        Ok(Box::new(file_type))
                    });
                    Some(Ok(Box::new(entry)))
                });
        }
    }
}


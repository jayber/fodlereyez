use std::error::Error;
use std::path::PathBuf;

use mockall::{mock, Sequence};

use super::file_system_proxy_traits::*;

mock! {
    pub(crate) MyReadDirProxy {}
    impl ReadDirProxy for MyReadDirProxy {
        fn path(&mut self) -> PathBuf;
    }
    impl Iterator for MyReadDirProxy {
        type Item = Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>;
        fn next(&mut self) -> Option<Result<Box<dyn DirPathEntryProxy>, Box<dyn Error>>>;
    }
}

pub(crate) fn set_expect(
    num_directories: usize, num_files: usize,
) -> Result<(PathBuf, MockFileSystemProxy), Box<dyn Error>> {
    let mut mock_file_operations = MockFileSystemProxy::new();
    let dir = PathBuf::from("current");
    let mut seq_read_dir = Sequence::new();
    expect_read_dir(num_directories, num_files, &mut mock_file_operations, dir.clone(), &mut seq_read_dir);
    mock_file_operations.expect_metadata().returning(|_| {
        let mut metadata = MockMetadataProxy::new();
        metadata.expect_len().return_const(1024 * 1024_u64);
        metadata.expect_file_attributes().return_const(0_u32);
        Ok(Box::new(metadata))
    });
    Ok((dir, mock_file_operations))
}

fn expect_read_dir(
    num_directories: usize, num_files: usize, mock_file_operations: &mut MockFileSystemProxy, buf: PathBuf,
    seq_read_dir: &mut Sequence,
) {
    mock_file_operations.expect_read_dir().times(1).in_sequence(seq_read_dir).returning(move |_dir| {
        let mut mock_read_dir = MockMyReadDirProxy::new();
        // mock_read_dir.expect_path().times(1).returning(|| PathBuf::from("test"));
        let mut seq = Sequence::new();
        for _i in 0..num_directories {
            expect_read_dir_next(true, &mut mock_read_dir, &mut seq);
        }
        for _j in 0..num_files {
            expect_read_dir_next(false, &mut mock_read_dir, &mut seq);
        }
        mock_read_dir.expect_next().times(1).in_sequence(&mut seq).returning(|| None);
        Ok(Box::new(mock_read_dir))
    });

    for _i in 0..num_directories {
        expect_read_dir(0, 0, mock_file_operations, buf.clone().clone(), seq_read_dir);
    }
}

fn expect_read_dir_next(is_dir: bool, mock_read_dir: &mut MockMyReadDirProxy, seq: &mut Sequence) {
    mock_read_dir.expect_next().times(1).in_sequence(seq).returning(move || {
        let mut entry = MockDirPathEntryProxy::new();
        entry.expect_path().returning(|| PathBuf::from("test"));
        entry.expect_file_type().returning(move || {
            let mut file_type = MockFileTypeProxy::new();
            file_type.expect_is_dir().times(1).return_const(is_dir);
            file_type.expect_is_symlink().times(1).return_const(false);
            Ok(Box::new(file_type))
        });
        Some(Ok(Box::new(entry)))
    });
}

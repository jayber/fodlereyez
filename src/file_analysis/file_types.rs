use std::path::{Path, PathBuf};
use std::{fmt, mem};

pub(crate) const ROLLUP_NAME: &str = "<other files...>";

#[derive(PartialEq, Debug)]
pub(crate) enum DirectoryEntry {
    File { len: Byteable, path: PathBuf },
    Folder { path: PathBuf, len: Byteable, entries: Vec<DirectoryEntry>, is_root: bool },
    Rollup { path: PathBuf, len: Byteable, entries: Vec<DirectoryEntry> }
}

//statics
impl DirectoryEntry {
    fn new_rollup(entries: Vec<DirectoryEntry>, path: PathBuf) -> DirectoryEntry {
        let len_sum = entries.iter().fold(0_u64, |a, b| b.len().0 + a);
        DirectoryEntry::Rollup { path, len: Byteable(len_sum), entries }
    }
    pub(crate) fn new_file(len: Byteable, path: PathBuf) -> DirectoryEntry { DirectoryEntry::File { len, path } }
}

impl DirectoryEntry {
    pub(crate) fn is_root(&self) -> bool {
        match self {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { is_root, .. } => *is_root,
            DirectoryEntry::Rollup { .. } => false
        }
    }
    pub(crate) fn path(&self) -> &Path {
        match self {
            DirectoryEntry::File { path, .. } => path.as_path(),
            DirectoryEntry::Folder { path, .. } => path.as_path(),
            DirectoryEntry::Rollup { path, .. } => path.as_path()
        }
    }
    pub(crate) fn entries(&self) -> Option<&Vec<DirectoryEntry>> {
        match self {
            DirectoryEntry::File { .. } => None,
            DirectoryEntry::Folder { entries, .. } => Some(entries),
            DirectoryEntry::Rollup { entries, .. } => Some(entries)
        }
    }
    pub(crate) fn rollup(&mut self) {
        match self {
            DirectoryEntry::File { .. } => {}
            DirectoryEntry::Rollup { .. } => {}
            DirectoryEntry::Folder { entries, path, .. } => {
                let mut old_entries = mem::take(entries);
                old_entries.sort_unstable_by_key(|a| a.len().0);

                let mut still_rolling_up = true;
                let mut files = vec![];
                for entry in old_entries {
                    if !entry.is_dir() && still_rolling_up {
                        files.push(entry);
                    } else {
                        still_rolling_up = false;
                        entries.push(entry);
                    }
                }

                if !files.is_empty() {
                    entries.push(DirectoryEntry::new_rollup(files, path.clone()));
                }
                entries.sort_unstable_by_key(|a| u64::MAX - a.len().0);
            }
        }
    }

    pub(crate) fn find(&self, match_path: &Path) -> Option<&Self> {
        fn find_entry<'a>(entries: &'a Vec<DirectoryEntry>, find_path: &Path) -> Option<&'a DirectoryEntry> {
            entries.iter().find(|&entry| find_path.starts_with(entry.path())).and_then(|entry| {
                if entry.path() == find_path {
                    Some(entry)
                } else {
                    entry.find(find_path)
                }
            })
        }

        if match_path == self.path() {
            Some(self)
        } else {
            match self {
                DirectoryEntry::File { .. } => None,
                DirectoryEntry::Folder { entries, .. } => find_entry(entries, match_path),
                DirectoryEntry::Rollup { entries, .. } => find_entry(entries, match_path)
            }
        }
    }

    pub(crate) fn get_parent(&self) -> Option<&Path> {
        match self {
            DirectoryEntry::File { path, .. } => path.parent(),
            DirectoryEntry::Folder { path, .. } => path.parent(),
            DirectoryEntry::Rollup { path, .. } => path.parent()
        }
    }
    /*

    pub(crate) fn get_parent(&self) -> PathBuf {
        match self {
            DirectoryEntry::File { path, .. } => path.clone(),
            DirectoryEntry::Folder { path, .. } => path.clone(),
            DirectoryEntry::Rollup { path, .. } => path.clone()
        }
    }*/
    pub fn has_children(&self) -> bool {
        match self {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { entries, .. } => !entries.is_empty(),
            DirectoryEntry::Rollup { .. } => true
        }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder { .. } => true,
            DirectoryEntry::Rollup { .. } => false
        }
    }
    pub fn len(&self) -> &Byteable {
        match self {
            DirectoryEntry::File { len, .. } => len,
            DirectoryEntry::Folder { len, .. } => len,
            DirectoryEntry::Rollup { len, .. } => len
        }
    }
    pub fn name(&self) -> String {
        fn get_file_name(buf: &PathBuf) -> String {
            buf.file_name().map_or(String::new(), |a| a.to_string_lossy().to_string())
        }
        match self {
            DirectoryEntry::File { path, .. } => get_file_name(path),
            DirectoryEntry::Folder { path, .. } => {
                let mut name = get_file_name(path);
                name.push(std::path::MAIN_SEPARATOR);
                name
            }
            DirectoryEntry::Rollup { .. } => String::from(ROLLUP_NAME)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Byteable(pub u64);

impl fmt::Display for Byteable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (scaled, size) = self.scale(0);
        let precision = if scaled.round() == scaled { 0 } else { 2 };
        let rounded = (100.0 * scaled).trunc() / 100.0;
        write!(f, "{:.*} {}", precision, rounded, size)
    }
}

const SCALES: [(u64, &str); 4] =
    [(1024 * 1024 * 1024 * 1024, "TB"), (1024 * 1024 * 1024, "GB"), (1024 * 1024, "MB"), (1024, "KB")];

impl Byteable {
    fn scale(&self, index: usize) -> (f64, &str) {
        if index == SCALES.len() {
            (self.0 as f64, "B")
        } else {
            let cur_scale = SCALES[index];
            let value = self.0 as f64 / cur_scale.0 as f64;
            if value >= 1.0 {
                (value, cur_scale.1)
            } else {
                self.scale(index + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod byteable {
        use crate::file_analysis::Byteable;

        #[test]
        fn test_byteable_output() {
            assert_eq!(Byteable(100).to_string(), "100 B");
            assert_eq!(Byteable(999).to_string(), "999 B");
            assert_eq!(Byteable(1000).to_string(), "1000 B");
            assert_eq!(Byteable(1024).to_string(), "1 KB");
            assert_eq!(Byteable((1024 * 1024) - 1).to_string(), "1023.99 KB");
            assert_eq!(Byteable(1024 * 1024).to_string(), "1 MB");
            assert_eq!(Byteable((1024 * 1024 * 1024) - 1).to_string(), "1023.99 MB");
            assert_eq!(Byteable(1024 * 1024 * 1024).to_string(), "1 GB");
        }
    }

    mod directory_entry {
        use std::path::PathBuf;

        use crate::file_analysis::file_types::DirectoryEntry;
        use crate::file_analysis::Byteable;

        mod rollup {
            use super::*;

            #[test]
            fn test_rollup() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::new_file(Byteable(0), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(1), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(2), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(3), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(4), PathBuf::new()));
                entries.push(DirectoryEntry::Folder {
                    path: PathBuf::new(),
                    len: Byteable(5),
                    entries: vec![],
                    is_root: false
                });
                let mut entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::new(), is_root: true };
                entry.rollup();
                let result = entry.entries().expect("no entries");
                assert_eq!(2, result.len());
            }

            #[test]
            fn test_rollup_nothing_to_roll() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::new_file(Byteable(6), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(7), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(8), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(9), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(10), PathBuf::new()));
                entries.push(DirectoryEntry::Folder {
                    path: PathBuf::new(),
                    len: Byteable(5),
                    entries: vec![],
                    is_root: false
                });
                let mut entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::new(), is_root: true };
                entry.rollup();
                let result = entry.entries().expect("no entries");
                assert_eq!(6, result.len());
            }

            #[test]
            fn test_rollup_with_both() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::new_file(Byteable(1), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(1), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(2), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(3), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(4), PathBuf::new()));

                entries.push(DirectoryEntry::Folder {
                    path: PathBuf::new(),
                    len: Byteable(5),
                    entries: vec![],
                    is_root: false
                });

                entries.push(DirectoryEntry::new_file(Byteable(6), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(7), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(8), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(9), PathBuf::new()));
                entries.push(DirectoryEntry::new_file(Byteable(10), PathBuf::new()));
                let mut entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::new(), is_root: true };
                entry.rollup();
                let result = entry.entries().expect("no entries");
                assert_eq!(7, result.len());
                match result.first().expect("first entry exists") {
                    DirectoryEntry::File { .. } => panic!("file found when expecting rollup"),
                    DirectoryEntry::Folder { .. } => panic!("folder found when expecting rollup"),
                    DirectoryEntry::Rollup { len, entries, .. } => {
                        assert_eq!(11, len.0);
                        assert_eq!(5, entries.len());
                    }
                }
            }
        }

        mod find {
            use super::*;

            #[test]
            fn test_find_self() {
                let entry = DirectoryEntry::Folder {
                    entries: vec![],
                    len: Byteable(0),
                    path: PathBuf::from("this"),
                    is_root: true
                };
                assert_eq!(&entry, entry.find(&PathBuf::from("this")).expect("to find self"));
            }

            #[test]
            fn test_find_self_with_more_than_one_same_name() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::Folder {
                    entries: vec![],
                    len: Byteable(0),
                    path: PathBuf::from("this"),
                    is_root: false
                });
                let entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::from("this"), is_root: true };
                assert_eq!(&entry, entry.find(&PathBuf::from("this")).expect("to find self"));
            }

            #[test]
            fn test_find_self_with_more_than_one_same_name2() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::Folder {
                    entries: vec![],
                    len: Byteable(10),
                    path: PathBuf::from("this\\this"),
                    is_root: false
                });
                let entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::from("this"), is_root: true };
                assert_eq!(10, entry.find(&PathBuf::from("this\\this")).expect("to find other").len().0);
            }

            #[test]
            fn test_find_self_with_more_than_one_different_name() {
                let mut entries = vec![];
                entries.push(DirectoryEntry::Folder {
                    entries: vec![],
                    len: Byteable(0),
                    path: PathBuf::from("that"),
                    is_root: false
                });
                let entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::from("this"), is_root: true };
                assert_eq!(&entry, entry.find(&PathBuf::from("this")).expect("to find self"));
            }

            #[test]
            fn test_find_self_with_more_than_one_different_name_2() {
                let mut entries = vec![];
                let that = DirectoryEntry::Folder {
                    entries: vec![],
                    len: Byteable(0),
                    path: PathBuf::from("that"),
                    is_root: false
                };
                entries.push(that);
                let entry =
                    DirectoryEntry::Folder { entries, len: Byteable(0), path: PathBuf::from("this"), is_root: true };
                assert_eq!("that\\", entry.find(&PathBuf::from("that")).expect("to find self").name());
            }
        }
    }
}

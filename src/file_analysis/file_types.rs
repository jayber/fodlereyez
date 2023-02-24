use std::path::PathBuf;
use std::{fmt, mem};

pub(crate) const ROLLUP_NAME: &str = "<other files...>";

pub(crate) enum DirectoryEntry {
    File { len: Byteable, path: PathBuf },
    Folder { path: PathBuf, len: Byteable, entries: Vec<DirectoryEntry> },
    Rollup { path: PathBuf, len: Byteable, entries: Vec<DirectoryEntry> }
}

//statics
impl DirectoryEntry {
    fn new_rollup(entries: Vec<DirectoryEntry>, path: PathBuf) -> DirectoryEntry {
        let len_sum = entries.iter().fold(0_u64, |a, b| b.len().val + a);
        DirectoryEntry::Rollup { path, len: Byteable { val: len_sum }, entries }
    }
    pub(crate) fn new_file(len: Byteable, path: PathBuf) -> DirectoryEntry { DirectoryEntry::File { len, path } }
}

impl DirectoryEntry {
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
                let mut old_entries = mem::replace(entries, Vec::new());
                old_entries.sort_by(|a, b| a.len().val.partial_cmp(&b.len().val).unwrap());

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
                entries.sort_unstable_by_key(|a| 0_i64 - a.len().val as i64);
            }
        }
    }

    pub(crate) fn find(&self, match_path: &PathBuf) -> Option<&Self> {
        fn find_entry<'a>(entries: &'a Vec<DirectoryEntry>, find_path: &PathBuf) -> Option<&'a DirectoryEntry> {
            entries.iter().find(|&entry| find_path.starts_with(entry.get_path_clone())).and_then(|entry| {
                if &entry.get_path_clone() == find_path {
                    Some(entry)
                } else {
                    entry.find(find_path)
                }
            })
        }

        if match_path == &self.get_path_clone() {
            Some(self)
        } else {
            match self {
                DirectoryEntry::File { .. } => None,
                DirectoryEntry::Folder { entries, .. } => find_entry(entries, match_path),
                DirectoryEntry::Rollup { entries, .. } => find_entry(entries, match_path)
            }
        }
    }

    pub(crate) fn get_path_clone(&self) -> PathBuf {
        match self {
            DirectoryEntry::File { path, .. } => path.clone(),
            DirectoryEntry::Folder { path, .. } => path.clone(),
            DirectoryEntry::Rollup { path, .. } => path.clone()
        }
    }

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

#[derive(Debug, Copy, Clone)]
pub(crate) struct Byteable {
    pub val: u64
}

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
            (self.val as f64, "B")
        } else {
            let cur_scale = SCALES[index];
            let value = self.val as f64 / cur_scale.0 as f64;
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
    use super::Byteable;

    #[test]
    fn test_byteable_output() {
        assert_eq!(Byteable { val: 100 }.to_string(), "100 B");
        assert_eq!(Byteable { val: 999 }.to_string(), "999 B");
        assert_eq!(Byteable { val: 1000 }.to_string(), "1000 B");
        assert_eq!(Byteable { val: 1024 }.to_string(), "1 KB");
        assert_eq!(Byteable { val: (1024 * 1024) - 1 }.to_string(), "1023.99 KB");
        assert_eq!(Byteable { val: (1024 * 1024) }.to_string(), "1 MB");
        assert_eq!(Byteable { val: (1024 * 1024 * 1024) - 1 }.to_string(), "1023.99 MB");
        assert_eq!(Byteable { val: (1024 * 1024 * 1024) }.to_string(), "1 GB");
    }
}

use std::path::PathBuf;
use std::{fmt, mem};

pub(crate) struct DirectoryTree {
    pub name: String,
    pub len: Byteable,
    pub entries: Vec<DirectoryEntry>
}

impl DirectoryTree {
    pub fn find(&self, path: &PathBuf) -> Option<&DirectoryTree> {
        self.entries
            .iter()
            .find(|&entry| match entry {
                DirectoryEntry::File { path: entry_path, .. } => path == entry_path,
                DirectoryEntry::Folder { path: entry_path, .. } => path.starts_with(entry_path),
                DirectoryEntry::Rollup { path: entry_path, .. } => path.starts_with(entry_path)
            })
            .map_or(Some(self), |entry| match entry {
                DirectoryEntry::File { .. } => {
                    // todo - i think only have to do this because not detecting match to current
                    // directory, which is because it isn't a DirectoryEntry, just a DirectoryTree
                    // panic!("Found a file while looking for a container")
                    Some(self)
                }
                DirectoryEntry::Folder { path: entry_path, branch, .. } => {
                    if path == entry_path {
                        Some(branch)
                    } else {
                        branch.find(path)
                    }
                }
                DirectoryEntry::Rollup { path: entry_path, branch, .. } => {
                    if path == entry_path {
                        Some(branch)
                    } else {
                        branch.find(path)
                    }
                }
            })
    }
    pub fn add_directory(&mut self, tree: DirectoryTree, path: PathBuf) {
        self.entries.push(DirectoryEntry::Folder { branch: tree, path });
    }
    pub fn add_file(&mut self, name: String, len: Byteable, path: PathBuf) {
        self.entries.push(DirectoryEntry::File { name, len, path });
    }
    pub fn new(name: String) -> Self { DirectoryTree { name, len: Byteable { val: 0_u64 }, entries: vec![] } }

    pub fn rollup(&mut self, path: PathBuf) {
        self.entries.sort_by(|a, b| a.len().val.partial_cmp(&b.len().val).unwrap());

        let mut files: Vec<DirectoryEntry> = vec![];
        let inner_entries = mem::take(&mut self.entries);
        let mut still = true;
        for entry in inner_entries {
            if !entry.is_dir() && still {
                files.push(entry);
            } else {
                still = false;
                self.entries.push(entry);
            }
        }

        if !files.is_empty() {
            self.entries.push(DirectoryEntry::new_rollup(files, path));
        }
        self.entries.sort_by(|a, b| b.len().val.partial_cmp(&a.len().val).unwrap());
    }
}

pub(crate) enum DirectoryEntry {
    File { name: String, len: Byteable, path: PathBuf },
    Folder { branch: DirectoryTree, path: PathBuf },
    Rollup { branch: DirectoryTree, path: PathBuf }
}

pub(crate) const ROLLUP_NAME: &str = "<other files...>";

impl DirectoryEntry {
    pub(crate) fn get_path(&self) -> PathBuf {
        match self {
            DirectoryEntry::File { path, .. } => path.clone(),
            DirectoryEntry::Folder { path, .. } => path.clone(),
            DirectoryEntry::Rollup { path, .. } => path.clone()
        }
    }
    fn new_rollup(entries: Vec<DirectoryEntry>, path: PathBuf) -> DirectoryEntry {
        let len_sum = entries.iter().fold(0_u64, |a, b| b.len().val + a);
        let tree = DirectoryTree { name: String::from("other files"), len: Byteable { val: len_sum }, entries };
        DirectoryEntry::Rollup { branch: tree, path }
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
            DirectoryEntry::File { name: _, len, .. } => len,
            DirectoryEntry::Folder { branch, .. } => &branch.len,
            DirectoryEntry::Rollup { branch, .. } => &branch.len
        }
    }
    pub fn name(&self) -> String {
        match self {
            DirectoryEntry::File { name, .. } => name.clone(),
            DirectoryEntry::Folder { branch, .. } => {
                branch.name.clone() + std::path::MAIN_SEPARATOR.to_string().as_str()
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

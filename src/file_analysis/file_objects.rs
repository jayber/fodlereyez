use std::{fmt, mem};

pub(crate) struct DirectoryTree {
    pub name: String,
    pub len: Byteable,
    pub entries: Vec<DirectoryEntry>
}

impl DirectoryTree {
    pub fn add_directory(&mut self, tree: DirectoryTree) { self.entries.push(DirectoryEntry::Folder(tree)); }
    pub fn add_file(&mut self, name: String, len: Byteable) { self.entries.push(DirectoryEntry::File { name, len }); }
    pub fn new(name: String) -> Self { DirectoryTree { name, len: Byteable { val: 0_u64 }, entries: vec![] } }

    pub fn rollup(&mut self) {
        self.entries.sort_by(|a, b| a.len().val.partial_cmp(&b.len().val).unwrap());

        let mut files: Vec<DirectoryEntry> = vec![];
        let inner_entries = mem::replace(&mut self.entries, vec![]);
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
            self.entries.push(DirectoryEntry::new_rollup(files));
        }
        self.entries.sort_by(|a, b| b.len().val.partial_cmp(&a.len().val).unwrap());
    }
}

pub(crate) enum DirectoryEntry {
    File { name: String, len: Byteable },
    Folder(DirectoryTree),
    Rollup { len: Byteable, entries: Vec<DirectoryEntry> }
}

pub(crate) const ROLLUP_NAME: &'static str = "<other files...>";

impl DirectoryEntry {
    fn new_rollup(entries: Vec<DirectoryEntry>) -> DirectoryEntry {
        let len_sum = entries.iter().fold(0_u64, |a, b| b.len().val + a);
        DirectoryEntry::Rollup { len: Byteable { val: len_sum }, entries }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            DirectoryEntry::File { .. } => false,
            DirectoryEntry::Folder(_) => true,
            DirectoryEntry::Rollup { .. } => false
        }
    }
    pub fn len(&self) -> &Byteable {
        match self {
            DirectoryEntry::File { name: _, len } => len,
            DirectoryEntry::Folder(dir) => &dir.len,
            DirectoryEntry::Rollup { len: val, .. } => val
        }
    }
    pub fn name(&self) -> String {
        match self {
            DirectoryEntry::File { name, len: _ } => name.clone(),
            DirectoryEntry::Folder(dir) => dir.name.clone() + std::path::MAIN_SEPARATOR.to_string().as_str(),
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

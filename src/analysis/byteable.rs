use std::fmt;

#[derive(Debug)]
pub struct Byteable {
    pub val: u64,
}

impl fmt::Display for Byteable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (scaled, size) = self.scale(0);
        let precision = if scaled.round() == scaled { 0 } else { 2 };
        let rounded = (100.0 * scaled).trunc() / 100.0;
        write!(f, "{:.*} {}", precision, rounded, size)
    }
}

const SCALES: [(u64, &'static str); 3] = [(1000000000, "GB"), (1000000, "MB"), (1000, "KB")];

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
        assert_eq!(Byteable { val: 1000 }.to_string(), "1 KB");
        assert_eq!(Byteable { val: 999999 }.to_string(), "999.99 KB");
        assert_eq!(Byteable { val: 1000000 }.to_string(), "1 MB");
        assert_eq!(Byteable { val: 999999999 }.to_string(), "999.99 MB");
        assert_eq!(Byteable { val: 1000000000 }.to_string(), "1 GB");
    }
}
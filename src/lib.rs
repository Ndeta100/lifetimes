#![warn(rust_2018_idioms)]
pub struct StrSplit {
    remainder: &str,
    delimeter: &str,
}
impl StrSplit {
    pub fn new(haystack: &str, delimeter: &str) -> Self {
        Self {
            remainder: haystack,
            delimeter,
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_deli) = self.remainder.find(self.delimeter) {
            let until_delimeter = &self.remainder[..next_deli];
            self.remainder = &self.remainder[(next_deli + self.remainder.len())..];
            Some(until_delimeter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

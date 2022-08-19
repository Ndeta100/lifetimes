#![warn(rust_2018_idioms)]
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimeter> {
    remainder: Option<&'haystack str>,
    delimeter: &'delimeter str,
}
impl<'haystack, 'delimeter> StrSplit<'haystack, 'delimeter> {
    pub fn new(haystack: &'haystack str, delimeter: &'delimeter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

impl<'haystack, 'delimeter> Iterator for StrSplit<'haystack, 'delimeter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        //impl<T> Option<T>{fn as_mut(&mut self)->Option<T}
        if let Some(next_delim) = remainder.find(self.delimeter) {
            let until_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimeter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}
pub fn until_char<'s>(s: &'s str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("strSpli alweays gives at least one results")
}
#[test]
fn until_char_test() {
    assert_eq!(until_char("Hello World", 'o'), "hell");
}
#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, "").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, "").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}
#[test]
fn empty_tail() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, "").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

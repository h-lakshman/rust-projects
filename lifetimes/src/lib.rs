#[derive(Debug)]
struct SplitStr<'a, D> {
    reminder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> SplitStr<'a, D> {
    fn new(haystack: &'a str, delimiter: D) -> Self {
        Self {
            reminder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, D> Iterator for SplitStr<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let reminder: &mut &str = self.reminder.as_mut()?;
        if let Some((start_delim, end_delim)) = self.delimiter.find_next(reminder) {
            let until_delim = &reminder[..start_delim];
            *reminder = &reminder[end_delim..];
            Some(until_delim)
        } else {
            self.reminder.take()
        }
    }
}
fn until_char(s: &str, c: char) -> &str {
    SplitStr::new(s, &*format!("{c}"))
        .next()
        .expect("StrSplit has atleast one word")
}
trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, (start + self.len())))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, (start + self.len_utf8())))
    }
}

#[test]
fn it_works() {
    let s = "a b c d";
    let y: Vec<_> = SplitStr::new(s, " ").collect();
    assert_eq!(y, vec!["a", "b", "c", "d"])
}
#[test]
fn tail() {
    let s = "a b c d e ";
    let y: Vec<_> = SplitStr::new(s, " ").collect();
    assert_eq!(y, vec!["a", "b", "c", "d", "e", ""]);
}

#[test]
fn unti_char_test() {
    let s = until_char("hello world", 'o');
    assert_eq!(s, "hell");
}

pub fn strok<'s>(s: &'_ mut &'s str, delimiter: char) -> &'s str {
    if let Some(position) = s.find(delimiter) {
        let prefix = &s[..position];
        *s = &s[position + delimiter.len_utf8()..];
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s: &'static str = "hello world";
        let result = strok(&mut s, ' ');
        // let z = &mut s;
        assert_eq!(result, "hello");
        assert_eq!(s, "world");
    }
}

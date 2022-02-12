#![cfg(test)]
#![allow(non_snake_case)]

fn as_string_iter<'a>(v: &'a [&str]) -> impl Iterator<Item = String> + 'a {
    v.iter().map(|s| (*s).to_string())
}

mod ParsedArgs {
    use crate::tests::as_string_iter;
    use crate::{Interactive, ParsedArgs};
    use std::iter;

    #[test]
    fn new_no_args() {
        let parsed_args = ParsedArgs::new(iter::empty::<String>());
        assert_eq!(parsed_args, Ok(ParsedArgs { mode: Interactive }));
    }

    #[test]
    fn new() {
        assert!(ParsedArgs::new(as_string_iter(&[""])).is_err());
    }
}

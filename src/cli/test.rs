#![cfg(test)]
#![allow(non_snake_case)]

use crate::cli;

#[test]
fn command() {
    cli::command().debug_assert();
}

mod ParsedArgs {
    use std::net::ToSocketAddrs;

    use clap::error::ErrorKind;
    use pretty_assertions_sorted::assert_eq;
    use test_case::test_case;

    use crate::{Dedicated, ParsedArgs};

    #[test_case(
        &[""],
        &ParsedArgs::Interactive)]
    #[test_case(
        &["", "--dedicated", "--listen", "[::]:2020"],
        &Dedicated {
            listen: "[::]:2020".to_socket_addrs().unwrap().next().unwrap()
        })]
    fn from_iterator__Ok(args: &[&str], expected: &ParsedArgs) {
        assert_eq!(ParsedArgs::from_iterator(args).unwrap(), *expected);
    }

    #[test_case(
        &["", "-V"],
        ErrorKind::DisplayVersion)]
    #[test_case(
        &["", "--version"],
        ErrorKind::DisplayVersion)]
    #[test_case(
        &["", "-h"],
        ErrorKind::DisplayHelp)]
    #[test_case(
        &["", "--help"],
        ErrorKind::DisplayHelp)]
    #[test_case(
        &["", "--dedicated", "--listen", "[::]"],
        ErrorKind::ValueValidation)]
    #[test_case(
        &["", "--unknown"],
        ErrorKind::UnknownArgument)]
    #[test_case(
        &["", "--listen", "0.0.0.0:2020"],
        ErrorKind::MissingRequiredArgument)]
    fn from_iterator__Err(args: &[&str], expected: ErrorKind) {
        assert_eq!(
            ParsedArgs::from_iterator(args)
                .err()
                .map(|e| e.kind())
                .unwrap(),
            expected
        );
    }
}

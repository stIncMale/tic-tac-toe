#![cfg(test)]
#![allow(non_snake_case)]

mod app {
    use crate::cli;

    #[test]
    fn app() {
        cli::app().debug_assert();
    }
}

mod ParsedArgs {
    use crate::{Dedicated, ParsedArgs};
    use pretty_assertions_sorted::assert_eq;
    use std::net::{SocketAddr, SocketAddrV4, ToSocketAddrs};
    use std::str::FromStr;

    #[test]
    fn from_iterator__interactive() {
        assert_eq!(
            ParsedArgs::from_iterator(&[""]).unwrap(),
            ParsedArgs::Interactive
        );
    }

    #[test]
    fn from_iterator__dedicated() {
        assert_eq!(
            ParsedArgs::from_iterator(&["", "--listen", "[::]:1234"]).unwrap(),
            Dedicated {
                listen: "[::]:1234".to_socket_addrs().unwrap().next().unwrap(),
                listen_console: None
            }
        );
    }

    #[test]
    fn from_iterator__dedicated_with_console() {
        assert_eq!(
            ParsedArgs::from_iterator(&[
                "",
                "--listen",
                "0.0.0.0:1111",
                "--listen-console",
                "127.0.0.1:2222"
            ])
            .unwrap(),
            Dedicated {
                listen: SocketAddr::from(SocketAddrV4::from_str("0.0.0.0:1111").unwrap()),
                listen_console: Some(SocketAddr::from(
                    SocketAddrV4::from_str("127.0.0.1:2222").unwrap()
                ))
            }
        );
    }
}

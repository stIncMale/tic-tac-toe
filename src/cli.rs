use crate::{Dedicated, Interactive};
use clap::{App, Arg, Error};
use std::ffi::OsString;
use std::net::SocketAddr;

mod cli_tests;

const LISTEN: &str = "listen";
const LISTEN_CONSOLE: &str = "listen-console";

fn app() -> App<'static> {
    const ABOUT: &str = "A multiplayer turn-based game. Can run in one of the two modes: \
        interactive, dedicated server. \
        The interactive mode is the default one and allows \
        playing offline against an AI, hosting a game, joining a game as a guest. \
        The dedicated server mode allows guests to join, find other players and play with them.\n\n\
        HOMEPAGE: <https://github.com/stIncMale/tic-tac-toe>.\n\n\
        WARNING: The project is being developed, so not all functionality is implemented. \
        Attempts to use unimplemented features result in the process termination.";
    App::new("tick-tack-toe")
        .about(ABOUT)
        .long_about(ABOUT)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .required(false)
                .takes_value(false)
                .long_help("Print help information."),
        )
        .arg(
            Arg::new(LISTEN)
                .long(LISTEN)
                .required(false)
                .takes_value(true)
                .forbid_empty_values(true)
                .long_help(
                    "The TCP socket address to listen on for game clients. \
                    If specified, the application starts as a dedicated server.",
                ),
        )
        .arg(
            Arg::new(LISTEN_CONSOLE)
                .long(LISTEN_CONSOLE)
                .required(false)
                .requires("listen")
                .takes_value(true)
                .forbid_empty_values(true)
                .long_help(
                    "The TCP socket address to listen on for web console clients. \
                    May be specified only if the application started as a dedicated server,\
                    i.e., if <listen> is specified. \
                    If specified, the application starts a web console.",
                ),
        )
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParsedArgs {
    Interactive,
    Dedicated {
        listen: SocketAddr,
        listen_console: Option<SocketAddr>,
    },
}

impl ParsedArgs {
    pub fn from_iterator<I, T>(args: I) -> Result<ParsedArgs, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = app().try_get_matches_from(args)?;
        if matches.is_present(LISTEN) {
            let listen: SocketAddr = matches.value_of_t(LISTEN)?;
            let listen_console = if matches.is_present(LISTEN_CONSOLE) {
                Some(matches.value_of_t::<SocketAddr>(LISTEN_CONSOLE)?)
            } else {
                None
            };
            Ok(Dedicated {
                listen,
                listen_console,
            })
        } else {
            Ok(Interactive)
        }
    }
}

use crate::{Dedicated, Interactive};
use clap::{crate_authors, crate_name, crate_version, value_parser, Arg, Command, Error};
use std::ffi::OsString;
use std::net::SocketAddr;

mod cli_tests;

const DEDICATED: &str = "dedicated";
const LISTEN: &str = "listen";

fn command<'a>() -> Command<'a> {
    let about = "\
        \n\
        A multiplayer turn-based game. \
        The game rules are simple an can be read at <https://en.wikipedia.org/wiki/Tic-tac-toe>.\n\
        \n\
        Can run in one of the two modes:\n  \
          * interactive;\n  \
          * dedicated server.\n\
        \n\
        Interactive mode:\n    \
            The interactive mode is the default one and allows playing offline against an AI,\
            hosting a game, joining a game as a guest.\n\
        \n\
        Dedicated server mode:\n    \
            The dedicated server mode allows guests to join, \
            find other players and play with them.\n\
        \n\
        Homepage:\n    \
            <https://github.com/stIncMale/tic-tac-toe>.\n\
        \n\
        Warning:\n    \
            The project is being developed, not all functionality is implemented.";
    Command::new(crate_name!())
        .version(crate_version!())
        .author(concat!("\nAuthors:\n    ", crate_authors!()))
        .about(about)
        .long_about(about)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .required(false)
                .takes_value(false)
                .long_help("Print short/long help."),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .required(false)
                .takes_value(false)
                .long_help("Print version."),
        )
        .arg(
            Arg::new(DEDICATED)
                .long(DEDICATED)
                .required(false)
                .takes_value(false)
                .long_help(
                    "Start in the dedicated server mode. \
                    If not specified, the application starts in the interactive mode.",
                ),
        )
        .arg(
            Arg::new(LISTEN)
                .long(LISTEN)
                .required(false)
                .requires(DEDICATED)
                .takes_value(true)
                .value_parser(value_parser!(SocketAddr))
                .default_value("127.0.0.1:2020")
                .long_help(
                    "The TCP socket address to listen on for game clients \
                    and web console requests.",
                ),
        )
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParsedArgs {
    Interactive,
    Dedicated { listen: SocketAddr },
}

impl ParsedArgs {
    /// # Errors
    ///
    /// When [`Command::try_get_matches_from()`] errors.
    ///
    /// # Panics
    ///
    /// If the source code has a bug.
    pub fn from_iterator<I, T>(args: I) -> Result<ParsedArgs, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let arg_matches = command().try_get_matches_from(args)?;
        if arg_matches.contains_id(DEDICATED) {
            let listen: SocketAddr = arg_matches
                .get_one::<SocketAddr>(LISTEN)
                .map_or_else(|| panic!("{LISTEN} should be present"), ToOwned::to_owned);
            Ok(Dedicated { listen })
        } else {
            Ok(Interactive)
        }
    }
}

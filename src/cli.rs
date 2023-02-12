use std::{ffi::OsString, net::SocketAddr};

use clap::{crate_authors, crate_description, value_parser, Arg, ArgAction, Command, Error};

use crate::{Dedicated, Interactive, APP_INFO};

mod test;

const DEDICATED: &str = "dedicated";
const LISTEN: &str = "listen";

fn command() -> Command {
    let about = format!(
    "{crate_description} The game rules can be read at <https://en.wikipedia.org/wiki/Tic-tac-toe>.\n\
        \n\
        Can run in one of the two modes:\n  \
          * interactive;\n  \
          * dedicated server.\n\
        \n\
        Interactive mode:\n    \
            The interactive mode is the default one and allows playing offline against an AI,\
            hosting a game, joining a game as a guest.\n\
        Dedicated server mode:\n    \
            The dedicated server mode allows guests to join, \
            find other players and play with them.\n\
        \n\
        Warning:\n    \
            The project is being developed, not all functionality is implemented.\n\
        \n\
        Homepage:\n    \
            <{homepage}>.",
        crate_description = crate_description!(),
        homepage = APP_INFO.homepage
    );
    Command::new(APP_INFO.name)
        .bin_name(&APP_INFO.exe)
        .version(APP_INFO.version)
        .author(concat!("\nAuthors:\n    ", crate_authors!()))
        .about(about)
        .help_template(
            "{name} {version}\n\
            \n\
            {about}{author}.\n\
            \n\
            {usage-heading}\n\
            {usage}\n\
            {all-args}",
        )
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .required(false)
                .action(ArgAction::Help)
                .help("Print help."),
        )
        .arg(
            Arg::new("version")
                .short('V')
                .long("version")
                .required(false)
                .action(ArgAction::Version)
                .help("Print version."),
        )
        .arg(
            Arg::new(DEDICATED)
                .long(DEDICATED)
                .required(false)
                .action(ArgAction::SetTrue)
                .help(
                    "Start in the dedicated server mode. \
                    If not specified, the application starts in the interactive mode.",
                ),
        )
        .arg(
            Arg::new(LISTEN)
                .long(LISTEN)
                .required(false)
                .requires(DEDICATED)
                .action(ArgAction::Set)
                .value_parser(value_parser!(SocketAddr))
                .default_value("127.0.0.1:2020")
                .help(
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
        if arg_matches.get_flag(DEDICATED) {
            let listen: SocketAddr = arg_matches
                .get_one::<SocketAddr>(LISTEN)
                .map_or_else(|| panic!("`{LISTEN}` must be present"), ToOwned::to_owned);
            Ok(Dedicated { listen })
        } else {
            Ok(Interactive)
        }
    }
}

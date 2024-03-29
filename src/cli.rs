use std::{ffi::OsString, net::SocketAddr};

use clap::{crate_authors, crate_description, value_parser, Arg, ArgAction, Command, Error};

use crate::{process::APP_METADATA, Dedicated, Interactive};

mod test;

const DEDICATED_ARG_ID: &str = "TODO-dedicated";
const LISTEN_ARG_ID: &str = "TODO-listen";

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
            The project is being developed, unimplemented functionality is marked with \"TODO\".\n\
        \n\
        Homepage:\n    \
            <{homepage}>.",
        crate_description = crate_description!(),
        homepage = APP_METADATA.homepage()
    );
    Command::new(APP_METADATA.name())
        .bin_name(APP_METADATA.exe())
        .version(APP_METADATA.version())
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
            Arg::new(DEDICATED_ARG_ID)
                .long(DEDICATED_ARG_ID)
                .required(false)
                .action(ArgAction::SetTrue)
                .help(
                    "Start in the dedicated server mode. \
                    If not specified, the application starts in the interactive mode.",
                ),
        )
        .arg(
            Arg::new(LISTEN_ARG_ID)
                .long(LISTEN_ARG_ID)
                .required(false)
                .requires(DEDICATED_ARG_ID)
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
    Dedicated(DedicatedArgs),
}

impl ParsedArgs {
    /// # Errors
    ///
    /// When [`Command::try_get_matches_from()`] errors.
    ///
    /// # Panics
    ///
    /// If the source code has a bug.
    pub fn try_from_iterator<I, T>(args: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let arg_matches = command().try_get_matches_from(args)?;
        if arg_matches.get_flag(DEDICATED_ARG_ID) {
            let listen: SocketAddr = arg_matches
                .get_one::<SocketAddr>(LISTEN_ARG_ID)
                .map_or_else(
                    || panic!("`{LISTEN_ARG_ID}` must be present"),
                    ToOwned::to_owned,
                );
            Ok(Dedicated(DedicatedArgs { listen }))
        } else {
            Ok(Interactive)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DedicatedArgs {
    listen: SocketAddr,
}

impl DedicatedArgs {
    #[must_use]
    pub fn _listen(&self) -> SocketAddr {
        self.listen
    }
}

use alloc::sync::Arc;
use core::sync::atomic::{AtomicBool, Ordering};
use std::{env, env::current_exe, ffi::OsStr};

use once_cell::sync::Lazy;

// TODO refactor when https://github.com/rust-lang/rust/issues/74465 is done,
// also remove the `once_cell` dependency.
pub static APP_METADATA: Lazy<AppMetadata> = Lazy::new(|| AppMetadata {
    name: "Tic-tac-toe",
    version: env!("CARGO_PKG_VERSION"),
    authors: env!("CARGO_PKG_AUTHORS"),
    homepage: env!("CARGO_PKG_REPOSITORY"),
    exe: {
        let mut exe = current_exe()
            .as_ref()
            .map(|path| path.file_name().unwrap())
            .map(OsStr::to_string_lossy)
            .unwrap_or("<game-executable>".into())
            .into_owned();
        exe.shrink_to_fit();
        exe.leak()
    },
});

/// Exit codes complementing the canonical ones in [`ExitCode`](std::process::ExitCode).
#[derive(Debug, Copy, Clone)]
pub struct MoreExitCode;

impl MoreExitCode {
    pub const INVALID_ARGS: u8 = 2;
}

#[derive(Debug)]
pub struct AppMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: &'static str,
    pub homepage: &'static str,
    pub exe: &'static str,
}

#[derive(Debug, Default)]
pub struct ExitSignal {
    received: AtomicBool,
}

impl ExitSignal {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_received(&self) -> bool {
        self.received.load(Ordering::SeqCst)
    }

    pub fn mark_received(&self) {
        self.received.store(true, Ordering::SeqCst);
    }
}

#[allow(clippy::std_instead_of_core)]
pub fn setup_panic() {
    human_panic::setup_panic!(Metadata {
        name: APP_METADATA.name.into(),
        version: APP_METADATA.version.into(),
        authors: APP_METADATA.authors.into(),
        homepage: APP_METADATA.homepage.into(),
    });
}

#[must_use]
pub fn setup_ctrlc_handler() -> Arc<ExitSignal> {
    let exit_signal = Arc::new(ExitSignal::new());
    {
        let exit_signal = Arc::clone(&exit_signal);
        ctrlc::set_handler(move || exit_signal.mark_received())
            .expect("setting a Ctlr+C handler should not fail");
    }
    exit_signal
}

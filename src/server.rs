use alloc::sync::Arc;
use core::error::Error;

use crate::{cli::ParsedArgs, process::ExitSignal};

// TODO introduce and accept the inner struct of ParsedArgs::Dedicated
pub fn run(_: ParsedArgs, _exit_signal: &Arc<ExitSignal>) -> Result<(), Box<dyn Error>> {
    todo!()
}

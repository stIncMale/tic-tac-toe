use alloc::sync::Arc;
use core::error::Error;

use crate::{cli::DedicatedArgs, process::ExitSignal};

pub fn run(_args: &DedicatedArgs, _exit_signal: &Arc<ExitSignal>) -> Result<(), Box<dyn Error>> {
    todo!()
}

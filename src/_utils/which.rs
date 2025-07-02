use std::{path::PathBuf, process::Command};

use crate::{_utils::run_command::ShellReturn, sh};

pub fn which(cmd: &str) -> Option<PathBuf> {
    let result: ShellReturn = sh!("command -v {}", cmd);
    match result.err_code {
        0 => Some(result.stdout.trim().into()),
        _ => None,
    }
}

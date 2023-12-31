// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use clap::Command;
use thiserror::Error;

/// Generate the CLI command structure
fn command() -> Command {
    Command::new("usysconf")
        .about("Univeral system configuration")
        .long_about("System configuration agent for modern Linux distributions to handle a variety of installation and removal triggers")
        .arg_required_else_help(true)
}

/// Process CLI arguments for usysconf binary
pub fn process() -> Result<(), Error> {
    let _ = command().get_matches();
    Err(Error::NotImplemented)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("hurrdurr not yet implemented")]
    NotImplemented,
}

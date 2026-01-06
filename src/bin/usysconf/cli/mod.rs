// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use clap::{Parser, Subcommand};
use thiserror::Error;

#[derive(Parser)]
#[command(author, version = None)]
#[command(about = "Universal system configuration")]
#[command(long_about = "System configuration agent for modern linux distributions to handle a variety of installation and removal triggers")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {}

/// Process CLI arguments for usysconf binary
pub fn process() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(_) => return Err(Error::NotImplemented),
        None => {}
    };

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("hurrdurr not yet implemented")]
    NotImplemented,
}

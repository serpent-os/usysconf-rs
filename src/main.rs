// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod cli;
mod paths;

use std::process;

fn main() -> Result<(), cli::Error> {
    let result = cli::run();
    if let Err(cli::Error::EarlyExit(code)) = &result {
        process::exit(*code)
    }
    result
}

// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod cli;

fn main() -> Result<(), cli::Error> {
    cli::process()
}

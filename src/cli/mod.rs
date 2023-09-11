// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;

/// Process CLI arguments for usysconf binary
pub fn process() -> Result<(), Error> {
    Err(Error::NotImplemented)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("hurrdurr not yet implemented")]
    NotImplemented,
}

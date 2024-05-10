// SPDX-FileCopyrightText: Copyright © 2020-2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::{cli, paths};

pub const NAME: &str = "path";

pub fn command() -> Command {
    Command::new(NAME)
        .about("path-oriented operations")
        .subcommand(
            Command::new(CMD_CHECK)
                .arg(Arg::new(ARG_PATH).required(true))
                .about("Check if the path requires triggers to be run again"),
        )
        .subcommand(
            Command::new(CMD_LIST_TRIGGERS)
                .arg(Arg::new(ARG_PATH).required(true))
                .about("List all triggers that handle this path"),
        )
        .subcommand_required(true)
}

pub fn run(args: &ArgMatches) -> Result<(), cli::Error> {
    match args.subcommand() {
        Some((CMD_CHECK, subargs)) => run_check(subargs),
        Some((CMD_LIST_TRIGGERS, subargs)) => run_list_triggers(subargs),
        _ => unreachable!(),
    }
}

const ARG_PATH: &str = "path";
const CMD_CHECK: &str = "check";
const CMD_LIST_TRIGGERS: &str = "list-triggers";

fn run_check(args: &ArgMatches) -> Result<(), cli::Error> {
    let path = args.get_one::<PathBuf>(ARG_PATH).unwrap();

    let db = cli::read_database(args);
    let entry = db.files().find(|entry| &entry.path == path);
    let outdated = match entry {
        Some(entry) => {
            let mtime = paths::file_mtime(&entry.path)?;
            mtime != entry.mtime
        }
        None => true,
    };
    if outdated {
        eprintln!("{} is out of date", path.to_string_lossy());
        return Err(cli::Error::EarlyExit(1));
    }
    Ok(())
}

fn run_list_triggers(args: &ArgMatches) -> Result<(), cli::Error> {
    let queried_path = args.get_one::<PathBuf>(ARG_PATH).unwrap();

    let triggers = cli::load_triggers(args)?;
    for trigger in triggers {
        for pattern in trigger.patterns.keys() {
            if pattern.matches(queried_path.to_string_lossy()).is_some() {
                println!("{}", trigger.name);
                break;
            }
        }
    }
    Ok(())
}

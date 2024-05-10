// SPDX-FileCopyrightText: Copyright © 2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod path;
mod trigger;

use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

use clap::{crate_version, value_parser, Arg, ArgMatches, Command};
use thiserror::Error;
use triggers::Trigger;

use crate::paths::PathDb;

/// Runs the application based on the command line arguments received.
pub fn run() -> Result<(), Error> {
    let matches = command().get_matches();
    match matches.subcommand() {
        Some((path::NAME, subargs)) => path::run(subargs),
        Some((trigger::NAME, subargs)) => trigger::run(subargs),
        _ => unreachable!(),
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),

    #[error("bad format")]
    Format(#[from] serde_yaml::Error),

    #[error("invalid trigger")]
    Trigger(#[from] triggers::Error),

    #[error("returning early")]
    EarlyExit(i32),
}

/// Generates the CLI command structure.
fn command() -> Command {
    Command::new("usysconf")
        .about("Universal System Configuration utility to handle installation and removal triggers")
        .version(crate_version!())
        .arg_required_else_help(true)
        .arg(
            Arg::new(FLAG_DATABASE)
                .short('d')
                .long("database")
                .global(true)
                .value_parser(value_parser!(PathBuf))
                .default_value(DATABASE_PATH)
                .help("Path of the database of known paths"),
        )
        .arg(
            Arg::new(FLAG_TRIGGER_DIR)
                .short('t')
                .long("trigger-dir")
                .global(true)
                .value_parser(value_parser!(PathBuf))
                .default_value(TRIGGERS_DIR)
                .help("Directory containing trigger files"),
        )
        .subcommand(path::command())
        .subcommand(trigger::command())
        .subcommand_required(true)
}

const FLAG_DATABASE: &str = "database";
const FLAG_TRIGGER_DIR: &str = "trigger-dir";

/// Default path of the path database.
const DATABASE_PATH: &str = "/var/lib/usysconf/paths";
// Default directory of trigger files.
const TRIGGERS_DIR: &str = "/usr/share/usysconf/triggers";

fn read_database(args: &ArgMatches) -> PathDb {
    let path = args.get_one::<PathBuf>(FLAG_DATABASE).unwrap();
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
    } else {
        Ok(())
    }
    .and_then(|_| File::options().read(true).open(path))
    .map(|file| PathDb::open(file).unwrap_or_default())
    // TODO: It's OK to return the default, but we should inform the users.
    .unwrap_or_default()
}

fn write_database(db: &PathDb, args: &ArgMatches) -> io::Result<()> {
    let path = args.get_one::<PathBuf>(FLAG_DATABASE).unwrap();
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
    } else {
        Ok(())
    }
    .and_then(|_| {
        File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
    })
    .and_then(|file| db.save(file))
}

fn load_triggers(args: &ArgMatches) -> Result<Vec<Trigger>, Error> {
    let path = args.get_one::<PathBuf>(FLAG_TRIGGER_DIR).unwrap();

    let mut triggers = vec![];
    for file in fs::read_dir(path)? {
        let file = File::open(file?.path())?;
        let trigger: Trigger = serde_yaml::from_reader(file)?;
        triggers.push(trigger);
    }
    Ok(triggers)
}

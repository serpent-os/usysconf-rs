// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use std::collections::BTreeSet;
use std::io;
use std::path;

use clap::{Arg, ArgMatches, Command};
use triggers::iterpaths;
use triggers::CompiledHandler;
use triggers::{self, Trigger};

use crate::{cli, paths};

pub const NAME: &str = "trigger";

pub fn command() -> Command {
    Command::new(NAME)
        .about("trigger-oriented operations")
        .subcommand(Command::new(CMD_LIST).about("List all triggers"))
        .subcommand(
            Command::new(CMD_RUN)
                .about("Run trigger(s)")
                .arg(
                    Arg::new(ARG_TRIGGER)
                        .num_args(0..)
                        .help("Optional trigger name(s). If none is passed, all triggers are run"),
                )
                .arg(
                    Arg::new(FLAG_FORCE)
                        .short('f')
                        .long("force")
                        .num_args(0)
                        .help("Run triggers even if paths involved are already up to date"),
                ),
        )
        .subcommand_required(true)
}

pub fn run(args: &ArgMatches) -> Result<(), cli::Error> {
    match args.subcommand() {
        Some((CMD_LIST, subargs)) => run_list(subargs),
        Some((CMD_RUN, subargs)) => run_run(subargs),
        _ => unreachable!(),
    }
}

const ARG_TRIGGER: &str = "trigger";
const CMD_LIST: &str = "list";
const CMD_RUN: &str = "run";
const FLAG_FORCE: &str = "force";

fn run_list(args: &ArgMatches) -> Result<(), cli::Error> {
    let triggers = cli::load_triggers(args)?;
    for trigger in triggers {
        println!("{}", trigger.name);
    }
    Ok(())
}

fn run_run(args: &ArgMatches) -> Result<(), cli::Error> {
    let trigger_names = args
        .get_many::<String>(ARG_TRIGGER)
        .unwrap_or_default()
        .collect::<Vec<_>>();
    let force_exec = args
        .get_one::<bool>(FLAG_FORCE)
        .copied()
        .unwrap_or_default();

    let mut triggers = cli::load_triggers(args)?;
    triggers.retain(|trig| {
        let is_wanted = if !trigger_names.is_empty() {
            trigger_names.contains(&&trig.name)
        } else {
            true
        };
        let inhibited = trig.is_inhibited().unwrap_or(false);
        if inhibited {
            println!("Skipping {} because of inhibitors", trig.name);
        }
        is_wanted && !inhibited
    });
    let mut db = cli::read_database(args);
    execute_triggers(
        triggers::DepGraph::from_iter(&triggers).iter(),
        &mut db,
        force_exec,
    )?;

    Ok(cli::write_database(&db, args)?)
}

fn execute_triggers<'a>(
    triggers: impl Iterator<Item = &'a Trigger>,
    db: &mut paths::PathDb,
    force: bool,
) -> io::Result<()> {
    for trigger in triggers {
        let discovered_paths = trigger
            .patterns
            .keys()
            .filter_map(|pat| glob::glob(&pat.pattern.to_std_glob()).ok())
            .flatten()
            .filter_map(|matc| matc.ok());
        let differences = db
            .extract_diff(&trigger.name, paths::file_set(discovered_paths).unwrap())
            .collect::<Vec<_>>();
        let outdated_paths = differences
            .iter()
            .filter(|file| force || file.is_modified())
            .map(|file| file.value().path.to_string_lossy().to_string());

        let handlers = iterpaths::compiled_handlers(trigger, outdated_paths);
        if handlers.is_empty() {
            println!("Skipping {}", trigger.name);
        } else {
            println!("Running {}", trigger.name);
            execute_handlers(&handlers)?;
        }

        db.insert(
            trigger.name.clone(),
            differences
                .into_iter()
                .filter(|file| !file.is_removed())
                .map(|file| file.value().clone())
                .collect(),
        );
    }
    Ok(())
}

fn execute_handlers(handlers: &BTreeSet<CompiledHandler>) -> io::Result<()> {
    for h in handlers {
        let output = h.run(path::Path::new("/"))?;
        if let Some(code) = output.status.code() {
            if code != 0 {
                eprintln!("Handler exited with non-zero status code: {h}");
                eprintln!("   Stdout: {}", String::from_utf8(output.stdout).unwrap());
                eprintln!("   Stderr: {}", String::from_utf8(output.stderr).unwrap());
            }
        } else {
            eprintln!("Failed to execute handler: {h}");
        }
    }
    Ok(())
}

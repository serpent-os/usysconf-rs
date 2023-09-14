use async_process::{Child, Command};
use serde::Deserialize;
use std::fmt::Display;
use std::io;
use std::process::ExitStatus;
use thiserror::Error;

use crate::osenv::OsEnv;

#[derive(Debug, Default, Deserialize)]
/// Task is a work unit for a Trigger.
pub struct Task {
    /// Textual description of what this task does.
    pub description: String,

    /// Absolute path of the binary file called.
    binary: String,

    /// Arguments passed to binary.
    args: Vec<String>,
}

impl Task {
    /// Runs this task in the background and returns a handle to it.
    pub fn run(&self) -> io::Result<Child> {
        Command::new(&self.binary).args(&self.args).spawn()
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.binary)?;
        for a in &self.args {
            write!(f, "{}", a)?;
        }
        Ok(())
    }
}

/// Trigger is a set of rules associated to a a list of paths.
/// If allowed to run, Trigger will perform a set of tasks in response
/// to the modified paths.
#[derive(Debug, Deserialize)]
pub struct Trigger {
    /// Lists of paths associated to this trigger.
    pub paths: Vec<String>,

    /// List of environments in which this trigger is forced to run.
    /// Normally Triggers should not run in special environments.
    force_envs: Vec<OsEnv>,

    /// List of trigger names to be run before this trigger.
    deps: Vec<String>,

    /// Whether tasks are independent from each other and
    /// can be run concurrently.
    concurrent: bool,

    /// Tasks that this trigger can perform.
    tasks: Vec<Task>,
}

impl Trigger {
    pub fn from_yaml(source: impl io::Read) -> Result<Self, Error> {
        serde_yaml::from_reader(source).map_err(Error::from)
    }

    pub fn run(&self) -> Result<(), Error> {
        if self.tasks.is_empty() {
            return Err(Error::NoTasks);
        }
        async_io::block_on(async {
            if self.concurrent {
                return self.run_parallel().await;
            }
            self.run_serial().await
        })
    }

    async fn run_parallel(&self) -> Result<(), Error> {
        let mut wait_list = Vec::with_capacity(self.tasks.len());
        for t in &self.tasks {
            wait_list.push(t.run()?.status());
        }
        let results = futures::prelude::future::try_join_all(wait_list).await?;
        for r in results {
            if !r.success() {
                return Err(Error::TaskFailed(r));
            }
        }
        Ok(())
    }

    async fn run_serial(&self) -> Result<(), Error> {
        for t in &self.tasks {
            let result = t.run()?.status().await?;
            if !result.success() {
                return Err(Error::TaskFailed(result));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to deserialize trigger")]
    Deserialization(#[from] serde_yaml::Error),

    #[error("there are no tasks specified for this trigger")]
    NoTasks,

    #[error("failed to spawn process")]
    Io(#[from] io::Error),

    #[error("task failed: {0}")]
    TaskFailed(ExitStatus),
}

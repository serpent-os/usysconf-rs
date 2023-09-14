use serde::Deserialize;
use std::io;
use std::process::{Child, Command};

/// Env describes a special status the operating system
/// may be in. Some Triggers are not allowed to run in certain
/// environments.
#[derive(Debug, Deserialize)]
pub enum Env {
    Chroot,
    Live,
}

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

/// Trigger is a set of rules associated to a a list of paths.
/// If allowed to run, Trigger will perform a set of tasks in response
/// to the modified paths.
#[derive(Debug, Deserialize)]
pub struct Trigger {
    /// Lists of paths associated to this trigger.
    pub paths: Vec<String>,

    /// List of environments in which this trigger won't run.
    skip_envs: Vec<Env>,

    /// List of trigger names to be run before this trigger.
    deps: Vec<String>,

    /// Whether tasks are independent from each other and
    /// can be run concurrently.
    concurrent: bool,

    /// Tasks that this trigger can perform.
    tasks: Vec<Task>,
}

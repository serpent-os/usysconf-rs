/// Env describes a special status the operating system
/// may be in. Some Triggers are not allowed to run in certain
/// environments.
pub enum Env {
    Chroot,
    Live,
}

/// Task is a work unit for a Trigger.
pub struct Task {
    /// Textual description of what this task does.
    description: String,

    /// Absolute path of the binary file called.
    binary: String,

    /// Arguments passed to binary.
    args: Vec<String>,
}

/// Trigger is a set of rules associated to a a list of paths.
/// If allowed to run, Trigger will perform a set of tasks in response
/// to the modified paths.
pub struct Trigger {
    /// Lists of paths associated to this trigger.
    paths: Vec<String>,

    /// List of environments in which this trigger won't run.
    skip_envs: Vec<Env>,

    /// List of trigger names to be run before this trigger.
    deps: Vec<String>,

    /// Tasks that this trigger can perform.
    tasks: Vec<Task>,
}

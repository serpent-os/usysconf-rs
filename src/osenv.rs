use std::os::linux::fs::MetadataExt;
use std::path::Path;
use std::{fs, io};

use serde::Deserialize;

const ROOT_FILE: &str = "proc/1/root";
const LIVE_FILE: &str = "run/livedev";

/// Env describes a special status the operating system may be in.
#[derive(Debug, Deserialize)]
pub enum OsEnv {
    Container,
    Live,
}

impl OsEnv {
    pub fn detect() -> io::Result<Option<Self>> {
        return Self::detect_from_root(Path::new("/"));
    }

    pub fn detect_from_root(root: &Path) -> io::Result<Option<Self>> {
        if Self::is_container(root)? {
            return Ok(Some(Self::Container));
        }
        if Self::is_live(root)? {
            return Ok(Some(Self::Live));
        }
        Ok(None)
    }

    fn is_container(root: &Path) -> io::Result<bool> {
        let proc_root = fs::metadata(root)?;
        let proc_meta = fs::metadata(root.join(ROOT_FILE))?;
        if proc_root.st_dev() != proc_meta.st_dev() {
            return Ok(true);
        }
        if proc_root.st_ino() != proc_meta.st_ino() {
            return Ok(true);
        }
        Ok(false)
    }

    fn is_live(root: &Path) -> io::Result<bool> {
        root.join(LIVE_FILE).try_exists()
    }
}

// SPDX-FileCopyrightText: Copyright © 2020-2023 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

mod diff;

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::iter::IntoIterator;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::time;

use thiserror::Error;

use self::diff::DiffOwned;

type TriggerName = String;
type Seconds = u64;
type FileSet = BTreeSet<File>;

#[derive(Default)]
pub struct PathDb {
    map: HashMap<TriggerName, FileSet>,
}

impl PathDb {
    pub fn open(src: impl Read) -> Result<Self, Error> {
        let mut src = BufReader::new(src);
        let mut buf = String::new();
        let mut map = HashMap::new();
        let mut curr_fileset = None;

        loop {
            buf.clear();
            if src.read_line(&mut buf)? == 0 {
                break;
            }

            let line = buf.trim_end();
            if buf.starts_with('\t') {
                curr_fileset
                    .as_deref_mut()
                    .ok_or_else(|| {
                        Error::DbFormat("database entry not associated to a trigger".to_string())
                    })
                    .and_then(|entries: &mut FileSet| {
                        let entry = File::deserialize(line.trim_start_matches('\t'))?;
                        Ok(entries.insert(entry))
                    })?;
            } else {
                curr_fileset = Some(map.entry(line.to_owned()).or_default());
            }
        }
        Ok(Self { map })
    }

    pub fn insert(&mut self, name: TriggerName, set: FileSet) {
        self.map.insert(name, set);
    }

    pub fn files(&self) -> impl Iterator<Item = &File> {
        self.map.values().flatten()
    }

    /// Extracts [`diff::FileDiff`] entries from the database.
    pub fn extract_diff(&mut self, name: &TriggerName, new: FileSet) -> DiffOwned {
        let old = self.map.remove(name).unwrap_or_default();
        DiffOwned {
            old,
            new: new.into_iter(),
        }
    }

    pub fn save(&self, writer: impl Write) -> io::Result<()> {
        let mut buf = String::new();
        let mut writer = BufWriter::new(writer);
        for (name, entries) in self.map.iter() {
            if entries.is_empty() {
                continue;
            }
            writeln!(writer, "{name}")?;
            for entry in entries {
                buf.clear();
                entry.serialize(&mut buf);
                writeln!(writer, "\t{}", buf)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct File {
    pub path: PathBuf,
    pub mtime: Seconds,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for File {}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.cmp(&other.path)
    }
}

impl File {
    fn deserialize(line: &str) -> Result<Self, Error> {
        let (path, mtime) = line
            .split_once(':')
            .ok_or_else(|| Error::DbFormat(line.to_string()))?;
        let path = PathBuf::from(path);
        let mtime = mtime.parse()?;
        Ok(Self { path, mtime })
    }

    fn serialize(&self, buf: &mut String) {
        buf.push_str(&self.path.to_string_lossy());
        buf.push(':');
        buf.push_str(&self.mtime.to_string());
    }
}

pub fn file_set(paths: impl IntoIterator<Item = PathBuf>) -> Result<FileSet, Error> {
    let mut set = FileSet::new();
    for path in paths {
        let mtime = file_mtime(&path)?;
        set.insert(File { path, mtime });
    }
    Ok(set)
}

/// Returns the modified time of a file.
pub fn file_mtime(path: &Path) -> io::Result<Seconds> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map(|mtime| {
            mtime
                .duration_since(time::UNIX_EPOCH)
                .unwrap_or(time::Duration::from_secs(0))
                .as_secs()
        })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("malformed database entry: {0}")]
    DbFormat(String),
    #[error("invalid integer: {0}")]
    InvalidInt(#[from] ParseIntError),
}

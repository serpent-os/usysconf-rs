use std::collections::btree_set::IntoIter;
use std::collections::BTreeSet;

use super::File;

/// A lazy iterator that extract [`File`]s from two sets
/// and produces [`FileDiff`]s.
pub struct DiffOwned {
    pub(crate) old: BTreeSet<File>,
    pub(crate) new: IntoIter<File>,
}

impl Iterator for DiffOwned {
    type Item = FileDiff;

    fn next(&mut self) -> Option<Self::Item> {
        match self.new.next() {
            Some(new_file) => {
                let old_file = self.old.take(&new_file);
                let modified = !old_file.is_some_and(|f| f.mtime == new_file.mtime);
                Some(FileDiff {
                    value: new_file,
                    modified,
                    removed: false,
                })
            }
            None => self.old.pop_first().map(|old_file| FileDiff {
                value: old_file,
                modified: true,
                removed: true,
            }),
        }
    }
}

pub struct FileDiff {
    value: File,
    modified: bool,
    removed: bool,
}

impl FileDiff {
    /// Returns the most recently known File value.
    pub fn value(&self) -> &File {
        &self.value
    }

    /// Returns whether the File was modified since the last time.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Returns whether the File was removed since the last time.
    pub fn is_removed(&self) -> bool {
        self.removed
    }
}

use std::collections::HashMap;
use std::io::{Read, Write};

type Path = String;
type Mtime = i64;

#[derive(Default)]
pub struct PathTimes {
    map: HashMap<Path, Mtime>,
}

impl PathTimes {
    pub fn new(source: impl Read) -> Result<Self, bincode::Error> {
        Ok(Self {
            map: bincode::deserialize_from(source)?,
        })
    }

    pub fn update(&mut self, path: Path, mtime: Mtime) {
        self.map.insert(path, mtime);
    }

    pub fn get(&self, path: &Path) -> Option<Mtime> {
        self.map.get(path).copied()
    }

    pub fn save(&mut self, writer: impl Write) -> Result<(), bincode::Error> {
        bincode::serialize_into(writer, &self.map)
    }
}

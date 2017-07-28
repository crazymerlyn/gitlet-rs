use std::path::Path;
use std;
use std::io::Write;

use errors::Result;

pub enum Contents {
    File(String),
    Folder(Vec<Entry>),
}

pub struct Entry {
    pub name: String,
    pub contents: Contents,
}

impl Entry {
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self.contents {
            Contents::File(ref text) => {
                let path = path.as_ref().join(&self.name);
                let mut file = std::fs::File::create(path)?;
                file.write_all(text.as_bytes())?;
            }
            Contents::Folder(ref elements) => {
                let path = path.as_ref().join(&self.name);
                std::fs::create_dir(&path)?;
                for elem in elements {
                    elem.write(&path)?;
                }
            }
        }
        Ok(())
    }
}

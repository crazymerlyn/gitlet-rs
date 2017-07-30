use std::path::Path;
use std;
use std::collections::HashMap;
use std::io::Write;

use errors::Result;

pub enum Contents {
    File(String),
    Folder(Entries),
}

pub type Entries = HashMap<String, Contents>;

pub fn write_tree<P: AsRef<Path>>(entries: Entries, path: P) -> Result<()> {
    for (name, contents) in entries.into_iter() {
        match contents {
            Contents::File(text) => {
                let path = path.as_ref().join(name);
                let mut file = std::fs::File::create(path)?;
                file.write_all(text.as_bytes())?;
            }
            Contents::Folder(elements) => {
                let path = path.as_ref().join(name);
                std::fs::create_dir(&path)?;
                write_tree(elements, &path)?;
            }
        }
    }
    Ok(())
}

#[macro_export]
macro_rules! files {
    ($($name:expr => $contents:expr),*) => {{
        let mut entries = Entries::new();
        $(entries.insert($name, $contents);)*
        entries
    }}
}


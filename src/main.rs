extern crate clap;

#[macro_use]
extern crate error_chain;

mod errors;
mod fs;

use fs::Entry;
use fs::Contents;

fn main() {
    let tree = Entry {
        name: ".gitlet".to_string(),
        contents: Contents::Folder(vec![
            Entry {
                name: "HEAD".to_string(),
                contents: Contents::File("ref: refs/heads/master\n".to_string()),
            },
        ]),
    };
    tree.write(".").unwrap();
    println!("Hello, world!");
}

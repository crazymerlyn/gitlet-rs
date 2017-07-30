#[macro_use]
extern crate error_chain;

#[macro_use]
mod fs;
mod errors;

use fs::Entries;
use fs::Contents;
use fs::write_tree;

pub fn init() {
    let tree = files! {
        "HEAD".to_string() => Contents::File("ref: refs/heads/master\n".to_string())
    };
    write_tree(tree, ".").unwrap();
}

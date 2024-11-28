use std::fs;
use std::path::Path;

fn main() {
    // Copy faces.json file for ascii faces.
    let source = Path::new("src/faces.json");
    let dest = Path::new("target/release/faces.json");

    fs::copy(source, dest).unwrap();
}

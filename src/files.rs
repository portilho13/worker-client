use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{self, Read, Write};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    File { path: String, content: Vec<u8> },
    Directory { path: String },
}

pub fn collect_entries(root_dir: String) -> io::Result<Vec<u8>> {
    let mut entries = Vec::new();

    for entry in WalkDir::new(root_dir) {
        let entry = entry?;
        let path = entry.path().to_string_lossy().to_string();

        if entry.path().is_dir() {
            entries.push(Entry::Directory { path });
        } else if entry.path().is_file() {
            let mut file = fs::File::open(entry.path())?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            entries.push(Entry::File { path, content });
        }
    }

    serialize_data(entries)
}

fn serialize_data(entries: Vec<Entry>) -> io::Result<Vec<u8>> {
    match serde_json::to_vec(&entries) {
        Ok(data) => Ok(data),
        Err(e) => {
            eprintln!("Serialization Error: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, e))
        }
    }
}

use chrono::Local;
use dirs_next::config_dir;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UrlType {
    url: String,
    tag: Option<String>,
    date: String,
}

pub enum FileMode {
    Read,
    Write,
}

pub fn get_url_file(mode: FileMode) -> (PathBuf, File) {
    let mut path = config_dir().expect("Unable to find .config directory");
    path.push("upchuk");

    std::fs::create_dir_all(&path).expect("Failed to create config directory");
    path.push("upchuk_urls.json");

    let url_file = match mode {
        FileMode::Write => OpenOptions::new().create(true).append(true).open(&path),
        FileMode::Read => OpenOptions::new().read(true).open(&path),
    }
    .expect("Failed to open file");

    (path, url_file)
}

pub fn add_urls(url: &str, tag: Option<&str>) {
    if url.is_empty() {
        return;
    }

    let url_entry = UrlType {
        url: url.trim().to_string(),
        tag: tag.map(|t| t.trim().to_string()),
        date: Local::now().format("%Y-%m-%d").to_string(),
    };

    let (_, mut url_file) = get_url_file(FileMode::Write);

    let json_line = serde_json::to_string(&url_entry).expect("Failed to serialze  url");

    writeln!(url_file, "{}", json_line).expect("Failed to add url");
}

// pub fn get_urls() {}

pub fn print_all_urls() {
    let (_, url_file) = get_url_file(FileMode::Read);
    let reader = BufReader::new(url_file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let entry: UrlType = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(e) => {
                println!("Failed to deserailes json entry: {}", e);
                continue;
            }
        };

        println!("URL: {}", entry.url);
        if let Some(tag) = entry.tag.as_deref() {
            println!("Tag: {}", tag);
        }
        println!("Date: {}", entry.date);
        println!("---");
    }
}

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

/// Returns the file path and file handle based on the given mode.
/// Ensures that the config directory exists. If reading and file doesn't exist,
/// it creates an empty file.
pub fn get_url_file(mode: FileMode) -> (PathBuf, File) {
    let mut path = config_dir().expect("Unable to find .config directory");
    path.push("upchuk"); // Create a subdirectory for this app

    // Ensure the upchuk directory exists
    std::fs::create_dir_all(&path).expect("Failed to create config directory");
    path.push("upchuk_urls.json"); // Define the filename

    // Open the file in either read or append mode
    let url_file = match mode {
        FileMode::Write => OpenOptions::new().create(true).append(true).open(&path),
        FileMode::Read => {
            if !path.exists() {
                println!("No urls found, Please add urls before checking");
                File::create(&path).expect("Failed to create empty urls json file");
            }
            OpenOptions::new().read(true).open(&path)
        }
    }
    .expect("Failed to open file");

    (path, url_file)
}

/// Adds a new URL entry with an optional tag to the file.
/// Automatically sets the current date.
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

/// Reads all URL entries from the file and returns them as a result.
/// Skips invalid JSON entries with a warning.
pub fn get_urls() -> Result<Vec<UrlType>, Box<dyn std::error::Error>> {
    let (_, url_file) = get_url_file(FileMode::Read);
    let reader = BufReader::new(url_file);

    let mut url_list: Vec<UrlType> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // Try to deserialize each JSON line into UrlType
        let entry: UrlType = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(e) => {
                println!("Failed to deserailes json entry: {}", e);
                continue; // Skip invalid entrie
            }
        };

        url_list.push(entry);
    }

    Ok(url_list)
}

/// Prints all URLs in a human-readable format with tag and date.
pub fn print_all_urls() {
    let urls = match get_urls() {
        Ok(urls) => urls,
        Err(e) => {
            println!("Error loading urls: {}", e);
            return;
        }
    };

    if urls.is_empty() {
        println!("No urls found");
        return;
    }

    for url in urls {
        println!("URL: {}", url.url);
        if let Some(tag) = url.tag.as_deref() {
            println!("Tag: {}", tag);
        }
        println!("Date: {}", url.date);
        println!("---");
    }
}

/// Iterates over all URLs and performs a GET request to check if they're reachable.
/// Prints success or failure for each URL independently.
pub fn check_all_urls() {
    let urls = match get_urls() {
        Ok(urls) => urls,
        Err(e) => {
            println!("Error loading urls: {}", e);
            return;
        }
    };

    if urls.is_empty() {
        println!("No urls found, add urls before checking");
        return;
    }

    for url in urls {
        match reqwest::blocking::get(&url.url) {
            Ok(response) => match response.text() {
                Ok(_e) => println!("{}\n", url.url),
                Err(e) => eprintln!("Failed to read body for {}: {}\n", url.url, e),
            },

            Err(e) => {
                print!("failed to get {:#?}: {}\n", &url.url, e);
            }
        };
    }
}

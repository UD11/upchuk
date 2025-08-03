use chrono::Local;
use dirs_next::config_dir;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write, stdout},
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
pub fn get_url_file(mode: FileMode) -> Result<(PathBuf, File), Box<dyn Error>> {
    let mut path = config_dir().ok_or("could not find config directory")?;
    path.push("upchuk"); // Create a subdirectory for this app

    // Ensure the upchuk directory exists
    std::fs::create_dir_all(&path)?;
    path.push("upchuk_urls.json");

    // Open the file in either read or append mode
    let url_file = match mode {
        FileMode::Write => OpenOptions::new().create(true).append(true).open(&path),
        FileMode::Read => {
            if !path.exists() {
                println!("No urls found, Please add urls before checking");
                File::create(&path)?;
            }
            OpenOptions::new().read(true).open(&path)
        }
    }?;

    Ok((path, url_file))
}

/// Adds a new URL entry with an optional tag to the file.
/// Automatically sets the current date.
pub fn add_urls(url: &str, tag: Option<&str>) -> Result<(), Box<dyn Error>> {
    if url.is_empty() {
        return Ok(());
    }

    if is_url_present(url)? {
        eprintln!("url is already present");
        return Ok(());
    }

    if tag.map(|t| t.contains(' ')).is_some() {
        eprintln!("tag cannot contain space");
        return Ok(());
    }

    let url_entry = UrlType {
        url: url.trim().to_string(),
        tag: tag.map(|t| t.trim().to_string()),
        date: Local::now().format("%Y-%m-%d").to_string(),
    };

    let (_, mut url_file) = get_url_file(FileMode::Write)?;

    let json_line = serde_json::to_string(&url_entry)?;

    writeln!(url_file, "{}", json_line)?;

    Ok(())
}

/// Reads all URL entries from the file and returns them as a result.
/// Skips invalid JSON entries with a warning.
pub fn get_urls() -> Result<Vec<UrlType>, Box<dyn Error>> {
    let (_, url_file) = get_url_file(FileMode::Read)?;
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
pub fn print_all_urls() -> Result<(), Box<dyn Error>> {
    let urls = get_urls()?;

    if urls.is_empty() {
        println!("No urls found");
        return Ok(());
    }

    for url in urls {
        println!("URL: {}", url.url);
        if let Some(tag) = url.tag.as_deref() {
            println!("Tag: {}", tag);
        }
        println!("Date: {}", url.date);
        println!("---");
    }

    Ok(())
}

/// Iterates over all URLs and performs a GET request to check if they're reachable.
/// Prints success or failure for each URL independently.
pub fn check_all_urls() -> Result<(), Box<dyn Error>> {
    let urls = match get_urls() {
        Ok(urls) => urls,
        Err(e) => {
            println!("Error loading urls: {}", e);
            return Ok(());
        }
    };

    if urls.is_empty() {
        println!("No urls found, add urls before checking");
        return Ok(());
    }

    for url in urls {
        print!("[…] {:30} ⏳ Checking... ", url.url);
        stdout().flush().unwrap();

        let start = std::time::Instant::now();

        match reqwest::blocking::get(&url.url) {
            Ok(resp) => {
                let duration = start.elapsed().as_millis();
                println!("\r[✓] {:30} ✔ {} in {}ms", url.url, resp.status(), duration);
            }
            Err(e) => {
                println!("\r[✗] {:30} ✖ Failed: {}", url.url, e.to_string());
            }
        }
    }

    Ok(())
}

fn is_url_present(target_url: &str) -> Result<bool, Box<dyn Error>> {
    let urls = get_urls()?;

    if urls.is_empty() {
        return Ok(false);
    }

    if urls.iter().find(|url| url.url == target_url).is_some() {
        return Ok(true);
    }

    Ok(false)
}

use dirs_next::config_dir;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub enum FileMode {
    Read,
    Write,
}

pub fn get_url_file(mode: FileMode) -> (PathBuf, File) {
    let mut path = config_dir().expect("Unable to find .config directory");
    path.push("upchuk");

    std::fs::create_dir_all(&path).expect("Failed to create config directory");
    path.push("upchuk_urls.txt");

    let url_file = match mode {
        FileMode::Write => OpenOptions::new().create(true).append(true).open(&path),
        FileMode::Read => OpenOptions::new().read(true).open(&path),
    }
    .expect("Failed to open file");

    (path, url_file)
}

pub fn add_urls(url: &str) {
    let (_, mut url_file) = get_url_file(FileMode::Write);
    writeln!(url_file, "{}", url).expect("Failed to add url");
}

// pub fn get_urls() {}

// file_watcher.rs
//
// Created on 2022/01/25 by Dante Ruiz
// Copyright 2022 Dante Ruiz
//
// Distributed under the MIT Lisense
// https://mit-license.org/

use std::{fs, time::SystemTime};
pub struct FileWatcher {
    file: String,
    last_modified: SystemTime,
}

impl FileWatcher {
    pub fn new(file: String) -> FileWatcher {
        let file_metadata = fs::metadata(file.clone());

        let last_modified = match file_metadata {
            Ok(metadata) => match metadata.modified() {
                Ok(modified) => modified,
                _ => SystemTime::now(),
            },
            _ => SystemTime::now(),
        };

        FileWatcher {
            file,
            last_modified,
        }
    }

    pub fn update<F: FnOnce()>(&mut self, callback: F) {
        let file_metadata = fs::metadata(self.file.clone());
        let modified = match file_metadata {
            Ok(metadata) => match metadata.modified() {
                Ok(modified) => modified,
                _ => self.last_modified.clone(),
            },
            _ => self.last_modified.clone(),
        };

        if self.last_modified.elapsed().unwrap().as_secs() > modified.elapsed().unwrap().as_secs() {
            self.last_modified = modified;
            callback();
        }
    }
}

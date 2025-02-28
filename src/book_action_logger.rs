use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

use crate::book::*;

enum LogEvent {
    Borrow{ book_title: String, username: String },
    Return{ book_title: String, username: String },
}

fn start_logger(rx: mpsc::Receiver<LogEvent>) {
    thread::spawn(move || {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("library_logs.txt")
            .expect("Failed to open file");

        while let Ok(event) = rx.recv() {
            let log_line = match event {
                LogEvent::Borrow{ book_title, username } => {
                    format!("Borrow: '{}', borrowed by {}", book_title, username)
                }
                LogEvent::Return{ book_title, username} => {
                    format!("Return: '{}', returned by {}", book_title, username)
                }
            };

            if let Err(e) = file.write_all(log_line.as_bytes()) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
    });
}
//Project B crates
use std::process::{Command, Stdio};
use std::str;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use crate::book::Book;
use rand::Rng;

pub fn new_book_shipment() ->  Result<Vec<Book>, Box<dyn Error + Send + Sync>>  {
    let mut rng = rand::rng();

    let output = Command::new("sh")
        .arg("-c")
        .arg("ls | grep '\\.csv'")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute ls");

    if !output.status.success() {
        return  Err(Box::<dyn Error + Send + Sync>::from("Error running search command..."));
    }

    let stdout = String::from_utf8(output.stdout).expect("Failed to parse output");

    let file_names: Vec<&str> = stdout.lines().collect();

    let mut library: Vec<Book> = Vec::new();

    for file in file_names {
        println!("Processing file: {}", file);
        let f = File::open(file).expect("Failed to open file");
        let reader = BufReader::new(f);

        for (i, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            if i == 0 { continue }

            let parts: Vec<&str> = line.split(',')
                .map(|s| s.trim())
                .collect();

            if parts.len() != 3 {
                eprintln!("Invalid CSV Format in file {}: {}", file, line);
                continue;
            }

            let mut num = rng.random_range(0..2);
            let borrow = match num {
                0 => true,
                1 => false,
                _ => false,
            };

            let book = Book{
                title: parts[0].into(),
                author: parts[1].into(),
                isbn: parts[2].into(),
                borrowed: borrow,
            };

            library.push(book);

        }

    };

    Ok(library)
}
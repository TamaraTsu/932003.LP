use std::env;
use std::fs::File;
use std::io::{Read, Write, stdout};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, sleep};
use std::time::Duration;
use reqwest::blocking::get;
use tee::TeeReader;

fn download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;

    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("file.bin");

    let file = File::create(file_name)?;

    let total_size = response.content_length().unwrap_or(0);
    println!(
        "Total download size {:.2} KB ({})",
        total_size as f64 / 1024.0,
        total_size
    );
    let downloaded = Arc::new(AtomicU64::new(0));
    let downloaded_clone = Arc::clone(&downloaded);

    let mut tee_reader = TeeReader::new(response, file);
    let mut buffer = [0; 1024];

    let worker = thread::spawn(move || {
        loop {
            let size = tee_reader.read(&mut buffer).unwrap();

            if size == 0 {
                break;
            }

            downloaded.fetch_add(size as u64, Ordering::Relaxed);
        }
    });

    let mut last_part = 0;
    loop {
        let part = downloaded_clone.load(Ordering::Relaxed);

        if part == total_size {
            break;
        }

        print!(
            "\rDownloading... {:.2}% ({:.2}/{:.2} KB) \t Speed: {:.2} KB/s",
            (part as f64 / total_size as f64) * 100.0,
            part as f64 / 1024.0,
            total_size as f64 / 1024.0,
            (part - last_part) as f64 / 1024.0
        );
        stdout().flush()?;
        last_part = part;
        sleep(Duration::from_secs(1));
    }

    worker.join().unwrap();
    println!("\nDone!");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./download <url>");
        return;
    }

    let url = &args[1];

    if let Err(err) = download(url) {
        eprintln!("Error: {}", err);
    }
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::Duration;

use website_checker::{run_concurrent, Config, WebsiteStatus};

/// Read list of URLs from file or stdin
fn read_urls(from_file: Option<PathBuf>) -> io::Result<Vec<String>> {
    let reader: Box<dyn BufRead> = if let Some(p) = from_file {
        Box::new(io::BufReader::new(File::open(p)?))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    let mut urls = Vec::new();
    for line in reader.lines() {
        let s = line?.trim().to_string();
        if !s.is_empty() && !s.starts_with('#') {
            urls.push(s);
        }
    }
    Ok(urls)
}

/// Print one WebsiteStatus as JSON (NDJSON line)
fn print_status_json(s: &WebsiteStatus) {
    println!("{}", serde_json::to_string(s).unwrap());
}

fn print_usage() {
    eprintln!(
        "Usage:
  website-checker --file=urls.txt [--timeout_ms=5000] [--workers=50] [--retries=0]
"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file: Option<PathBuf> = None;
    let mut timeout_ms: u64 = 5000;
    let mut workers: usize = 50;
    let mut retries: usize = 0;

    for arg in std::env::args().skip(1) {
        if let Some(v) = arg.strip_prefix("--file=") {
            file = Some(PathBuf::from(v));
        } else if let Some(v) = arg.strip_prefix("--timeout_ms=") {
            timeout_ms = v.parse().unwrap_or_else(|_| {
                print_usage();
                std::process::exit(2);
            });
        } else if let Some(v) = arg.strip_prefix("--workers=") {
            workers = v.parse().unwrap_or_else(|_| {
                print_usage();
                std::process::exit(2);
            });
        } else if let Some(v) = arg.strip_prefix("--retries=") {
            retries = v.parse().unwrap_or_else(|_| {
                print_usage();
                std::process::exit(2);
            });
        } else if arg == "--help" || arg == "-h" {
            print_usage();
            return Ok(());
        }
    }

    let urls = read_urls(file)?;
    if urls.is_empty() {
        print_usage();
        eprintln!("Provide URLs via --file=path or stdin (one per line).");
        std::process::exit(2);
    }

    let cfg = Config {
        timeout: Duration::from_millis(timeout_ms),
        workers,
        max_retries: retries,
    };

    // One-shot concurrent run (channel close → workers exit)
    let statuses = run_concurrent(urls, &cfg);
    for s in statuses {
        print_status_json(&s);
    }

    Ok(())
}
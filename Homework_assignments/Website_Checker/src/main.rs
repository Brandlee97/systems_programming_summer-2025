use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::Duration;

use website_checker::{Config, WebsiteStatus, run_concurrent};

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

/// Print one WebsiteStatus as JSON
fn print_status_json(s: &WebsiteStatus) {
    println!("{}", serde_json::to_string(s).unwrap());
}

/// Main: parse args, build config, run checks
fn main() -> anyhow::Result<()> {
    let mut file: Option<PathBuf> = None;
    let mut timeout_ms: u64 = 5000;
    let mut workers: usize = 50;
    let mut retries: usize = 0;

    // parse command-line args
    for arg in std::env::args().skip(1) {
        if let Some(v) = arg.strip_prefix("--file=") {
            file = Some(PathBuf::from(v));
        } else if let Some(v) = arg.strip_prefix("--timeout_ms=") {
            timeout_ms = v.parse().unwrap_or(timeout_ms);
        } else if let Some(v) = arg.strip_prefix("--workers=") {
            workers = v.parse().unwrap_or(workers);
        } else if let Some(v) = arg.strip_prefix("--retries=") {
            retries = v.parse().unwrap_or(retries);
        }
    }

    // load URLs
    let urls = read_urls(file)?;
    if urls.is_empty() {
        eprintln!("Provide URLs via --file=path or stdin (one per line).");
        std::process::exit(2);
    }

    // build config
    let cfg = Config {
        timeout: Duration::from_millis(timeout_ms),
        workers,
        max_retries: retries,
    };

    // run concurrent checks
    let statuses = run_concurrent(urls, &cfg);
    for s in statuses {
        print_status_json(&s);
    }

    Ok(())
}
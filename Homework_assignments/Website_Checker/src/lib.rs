use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Config {
    pub timeout: Duration,
    pub workers: usize,
    pub max_retries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteStatus {
    pub url: String,
    pub status: Result<u16, String>,
    pub response_time: Duration,
    pub timestamp: DateTime<Utc>,
}

/// Check one URL once and return status info
pub fn check_url_once(url: &str, cfg: &Config) -> WebsiteStatus {
    let client = reqwest::blocking::Client::builder()
        .timeout(cfg.timeout)
        .build();

    let start = Instant::now();
    let ts = Utc::now();

    let status = match client {
        Ok(c) => c
            .get(url)
            .send()
            .map(|r| r.status().as_u16())
            .map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: start.elapsed(),
        timestamp: ts,
    }
}

/// Retry a URL check until it works or max_retries is reached
fn check_with_retries(url: &str, cfg: &Config) -> WebsiteStatus {
    let mut last = check_url_once(url, cfg);
    let mut attempts = 0;
    while attempts < cfg.max_retries && last.status.is_err() {
        attempts += 1;
        last = check_url_once(url, cfg);
    }
    last
}

/// Run all URL checks with a worker thread pool + channels
pub fn run_concurrent(urls: Vec<String>, cfg: &Config) -> Vec<WebsiteStatus> {
    let total = urls.len();

    let (work_tx, work_rx) = mpsc::channel::<String>();
    let (res_tx, res_rx) = mpsc::channel::<WebsiteStatus>();

    // Share the receiver so all worker threads can pull jobs
    let shared_rx = Arc::new(Mutex::new(work_rx));

    // Spawn worker threads
    let mut handles = Vec::with_capacity(cfg.workers);
    for _ in 0..cfg.workers {
        let rx = Arc::clone(&shared_rx);
        let tx = res_tx.clone();
        let cfg_cloned = cfg.clone();

        let handle = thread::spawn(move || {
            loop {
                // get a job (URL) or exit if none left
                let msg = {
                    let guard = rx.lock().unwrap();
                    guard.recv()
                };

                let url = match msg {
                    Ok(u) => u,
                    Err(_) => break, // no more jobs
                };

                let status = check_with_retries(&url, &cfg_cloned);
                if tx.send(status).is_err() {
                    break;
                }
            }
        });
        handles.push(handle);
    }
    drop(res_tx); // close main sender

    // Send all URLs as jobs
    for u in urls {
        let _ = work_tx.send(u);
    }
    drop(work_tx); // close work channel

    // Collect results
    let mut out = Vec::with_capacity(total);
    for _ in 0..total {
        if let Ok(status) = res_rx.recv() {
            out.push(status);
        }
    }

    // Wait for workers to finish
    for h in handles {
        let _ = h.join();
    }

    out
}
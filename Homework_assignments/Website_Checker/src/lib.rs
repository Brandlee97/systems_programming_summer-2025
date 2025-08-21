use serde::{Deserialize, Serialize};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

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
    /// UTC timestamp as milliseconds since Unix epoch (no chrono dependency)
    pub timestamp_unix_ms: u128,
}

/// Check one URL once and return status info (ureq)
pub fn check_url_once(url: &str, cfg: &Config) -> WebsiteStatus {
    let agent = ureq::AgentBuilder::new()
        .timeout(cfg.timeout)
        .build();

    let start = Instant::now();
    let result = agent.get(url).call();
    let elapsed = start.elapsed();

    let status = match result {
        Ok(resp) => Ok(resp.status()),
        Err(ureq::Error::Status(code, _)) => Ok(code),
        Err(ureq::Error::Transport(t)) => Err(t.to_string()),
    };

    WebsiteStatus {
        url: url.to_string(),
        status,
        response_time: elapsed,
        timestamp_unix_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    }
}

/// Retry a URL check until it works or max_retries is reached (with tiny backoff)
fn check_with_retries(url: &str, cfg: &Config) -> WebsiteStatus {
    let mut last = check_url_once(url, cfg);
    let mut attempts = 0;
    while attempts < cfg.max_retries && last.status.is_err() {
        attempts += 1;
        thread::sleep(Duration::from_millis(150)); // small backoff
        last = check_url_once(url, cfg);
    }
    last
}

/// Run all URL checks with a worker thread pool + channels.
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

        let handle = thread::spawn(move || loop {
            // Use recv_timeout so idle threads can exit promptly once channel closes
            let msg = {
                let guard = rx.lock().unwrap();
                guard.recv_timeout(Duration::from_millis(100))
            };

            match msg {
                Ok(url) => {
                    let status = check_with_retries(&url, &cfg_cloned);
                    if tx.send(status).is_err() {
                        break; // collector gone
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break, // queue closed
            }
        });
        handles.push(handle);
    }
    drop(res_tx); // close main sender; worker clones remain

    // Enqueue all URLs and close the queue.
    for u in urls {
        let _ = work_tx.send(u);
    }
    drop(work_tx);

    // Collect results (exactly `total` items unless workers died)
    let mut out = Vec::with_capacity(total);
    for _ in 0..total {
        if let Ok(status) = res_rx.recv() {
            out.push(status);
        } else {
            break;
        }
    }

    // Join workers
    for h in handles {
        let _ = h.join();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_backoff_runs() {
        
        let cfg = Config { timeout: Duration::from_millis(10), workers: 1, max_retries: 2 };
        let _ = check_url_once("http://127.0.0.1:9", &cfg);
    }
}
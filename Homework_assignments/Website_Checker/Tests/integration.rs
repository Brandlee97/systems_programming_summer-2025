use httpmock::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use website_checker::{run_concurrent, Config};

#[test]
fn success_timeout_and_404() {
    let server = MockServer::start();

    let _ok = server.mock(|when, then| {
        when.method(GET).path("/ok");
        then.status(200).body("hi");
    });

    let _slow = server.mock(|when, then| {
        when.method(GET).path("/slow");
        then.status(200).delay(std::time::Duration::from_millis(300));
    });

    let _notfound = server.mock(|when, then| {
        when.method(GET).path("/nope");
        then.status(404);
    });

    let urls = vec![
        format!("{}/ok", server.base_url()),
        format!("{}/slow", server.base_url()),
        format!("{}/nope", server.base_url()),
    ];

    let cfg = Config {
        timeout: Duration::from_millis(100),
        workers: 8,
        max_retries: 1,
    };

    let stop = Arc::new(AtomicBool::new(false));
    let results = run_concurrent(urls, &cfg, stop);

    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|r| r.url.ends_with("/ok") && r.status == Ok(200)));
    assert!(results.iter().any(|r| r.url.ends_with("/nope") && r.status == Ok(404)));
    assert!(results.iter().any(|r| r.url.ends_with("/slow") && r.status.is_err()));
}

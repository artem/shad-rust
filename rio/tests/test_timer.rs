use test_log::test;

use std::time::{Duration, Instant};

////////////////////////////////////////////////////////////////////////////////

#[rio::test]
async fn test_simple() {
    let start = Instant::now();
    let duration = Duration::from_secs(1);
    rio::sleep(duration).await;
    assert!(start.elapsed() >= duration);
}

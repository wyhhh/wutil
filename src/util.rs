use std::time::Duration;
use std::time::Instant;

pub fn time_test(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

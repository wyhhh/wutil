use std::time::Duration;
use std::time::Instant;

pub fn time_test<R>(f: impl FnOnce() -> R) -> (Duration,R) {
    let start = Instant::now();
    let ret = f();
    (start.elapsed(), ret)
}

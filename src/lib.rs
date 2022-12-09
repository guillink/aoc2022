use std::time::{Duration, Instant};

pub fn time<F, R>(f: F) -> (R, Duration)
where
    F: Fn() -> R,
{
    let now = Instant::now();
    return (f(), now.elapsed());
}

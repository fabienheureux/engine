use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct Time {
    pub dt: f64,
    pub now: Duration,
    pub last_time: Duration,
}

impl Time {
    #[allow(unused)]
    pub fn now_to_secs(&self) -> f64 {
        Self::duration_to_secs(self.now)
    }

    pub fn now() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards.")
    }

    pub fn duration_to_secs(duration: Duration) -> f64 {
        duration.as_secs() as f64
            + f64::from(duration.subsec_nanos()) / f64::from(1_000_000_000)
    }
}

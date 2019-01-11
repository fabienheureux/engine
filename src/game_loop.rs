use std::time::{Duration, SystemTime, SystemTimeError};

// 16.6ms per frame for 60 frames per second.
const FPS: i32 = 60;

#[derive(Default)]
pub struct GameLoop {
    frame_rate: i32,
    is_running: bool,
    // Time stuff
    last_time: Option<SystemTime>,
    current_time: Option<SystemTime>,
}

pub fn now() -> SystemTime {
    SystemTime::now()
}

impl GameLoop {
    pub fn new() -> Self {
        Self {
            frame_rate: FPS,
            ..Self::default()
        }
    }

    pub fn run(&mut self, mut update: impl FnMut(f64) -> bool) {
        self.is_running = true;

        self.last_time = None;
        self.current_time = Some(now());

        while self.is_running {
            self.update_time();
            let dt = self.compute_dt();

            self.is_running = update(dt);

            self.sync_loop();
        }
    }

    // Synchronize loop to draw stuff at 60FPS.
    //
    // This function will sleep the main thread only if the current tick took less than
    // 16.6ms to complete. If not, do nothing (yet).
    fn sync_loop(&mut self) {
        let now = self.current_time.unwrap();
        let sleep_time =
            Self::compute_sleep_duration(self.frame_rate, now).unwrap();

        // The `checked_sub` method return `None` for negative result.
        if sleep_time.checked_sub(Duration::default()).is_some() {
            std::thread::sleep(sleep_time);
        } else {
            println!("Frame drop occurs here...");
        }
    }

    fn update_time(&mut self) {
        self.last_time = self.current_time;
        self.current_time = Some(now());
    }

    fn compute_dt(&self) -> f64 {
        if self.last_time.is_none() || self.current_time.is_none() {
            unimplemented!();
        }

        // Safely unwrapping time values here.
        let last_time = self.last_time.unwrap();
        let current_time = self.current_time.unwrap();

        let dt = current_time.duration_since(last_time);

        if dt.is_err() {
            unimplemented!();
        }

        let dt = dt.unwrap();
        // Cast Duration into float secs.
        return (dt.as_secs() as f64)
            + (dt.subsec_nanos() as f64) / (1_000_000_000 as f64);
    }

    /// Compute sleep duration from a given SystemTime.
    /// This function will return `None` if the computed duration
    /// is negative.
    fn compute_sleep_duration(
        frame_rate: i32,
        from_time: SystemTime,
    ) -> Result<Duration, SystemTimeError> {
        let ms_per_frame = Duration::from_millis((1000 / frame_rate) as u64);
        let elapsed = from_time.elapsed()?;

        // ELAPSED + MS_PER_FRAME - NOW
        let computed_tine =
            (elapsed + ms_per_frame) - now().duration_since(from_time)?;

        Ok(computed_tine)
    }
}

use crate::utils;
use std::time::SystemTime;

// 16.6ms per frame for 60 frames per second.
const FPS: i32 = 60;

#[derive(Default)]
pub struct GameLoop {
    is_running: bool,
    is_paused: bool,
}

impl GameLoop {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, mut process: impl FnMut() -> bool) {
        self.is_running = true;

        while self.is_running {
            let now = utils::now();
            self.is_paused = false;

            self.is_running = process();

            self.sync_loop(now);
        }
    }

    // Synchronize loop to run at 60FPS.
    //
    // This function will sleep the main thread only if the current tick took less than
    // 16.6ms to complete. If not, do nothing (yet).
    fn sync_loop(&mut self, now: SystemTime) {
        let sleep_time = utils::compute_sleep_duration(now, FPS).unwrap();
        if sleep_time.is_some() {
            let time = sleep_time.unwrap();

            self.is_paused = true;
            std::thread::sleep(time);
        } else {
            println!("Frame drop occurs here...");
        }
    }
}

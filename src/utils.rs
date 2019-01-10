use std::time::{Duration, SystemTime, SystemTimeError};

pub fn now() -> SystemTime {
    SystemTime::now()
}

/// Compute sleep duration from a given SystemTime.
/// This function will return `None` if the computed duration
/// is negative.
pub fn compute_sleep_duration(
    from_time: SystemTime,
    fps: i32,
) -> Result<Option<Duration>, SystemTimeError> {
    let ms_per_frame = std::time::Duration::from_millis((1000 / fps) as u64);

    let elapsed = from_time.elapsed().unwrap();

    let sleep_time = elapsed
        .checked_add(ms_per_frame)
        .unwrap()
        .checked_sub(now().duration_since(from_time)?);

    Ok(sleep_time)
}

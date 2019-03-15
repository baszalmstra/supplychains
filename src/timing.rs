use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    /// Time elapsed since the last frame in seconds
    delta_seconds: f32,

    /// Time elapsed since the last frame
    delta_time: Duration,

    /// The current frame number
    frame_number: u64,

    /// Time elapsed since game start
    absolute_time: Duration,
}

impl Time {
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn frame_number(&self) -> u64 {
        self.frame_number
    }

    pub fn absolute_time(&self) -> Duration {
        self.absolute_time
    }

    pub fn set_delta_time(&mut self, time: Duration) {
        self.delta_seconds = duration_to_secs(time);
        self.delta_time = time;

        self.absolute_time += time;
    }

    pub fn increment_frame_number(&mut self) {
        self.frame_number += 1;
    }
}

impl Default for Time {
    fn default() -> Self {
        Time {
            delta_seconds: 0.0,
            delta_time: Duration::default(),
            frame_number: 0,
            absolute_time: Duration::default(),
        }
    }
}

pub fn duration_to_secs(duration: Duration) -> f32 {
    duration.as_secs() as f32 + (duration.subsec_nanos() as f32 / 1.0e9)
}

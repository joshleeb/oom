use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Animation<T> {
    frames: Vec<T>,
    delta_time: Duration,

    frame_index: usize,
    last_update_time: SystemTime,
}

impl<T> Animation<T> {
    pub fn new(frames: Vec<T>, delta_time: Duration) -> Self {
        Animation {
            frames,
            delta_time,

            frame_index: 0,
            last_update_time: UNIX_EPOCH,
        }
    }

    pub fn next_frame(&mut self) -> Option<&T> {
        if let Ok(since) = SystemTime::now().duration_since(self.last_update_time) {
            if since < self.delta_time {
                return None;
            }
        }

        self.last_update_time = SystemTime::now();
        let frame = self.frames.get(self.frame_index);

        self.frame_index = (self.frame_index + 1) % self.frames.len();
        frame
    }
}

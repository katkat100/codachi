use std::time::{Duration, Instant};
use crate::state::Mood;
use crate::ui::sprites::{get_idle_frames, SpriteFrame};

pub struct AnimationState {
    current_frame: usize,
    last_frame_time: Instant,
    frame_duration: Duration,
    remark_text: Option<String>,
    remark_start: Option<Instant>,
    remark_duration: Duration,
}

impl AnimationState {
    pub fn new() -> Self {
        Self {
            current_frame: 0,
            last_frame_time: Instant::now(),
            frame_duration: Duration::from_millis(250),
            remark_text: None,
            remark_start: None,
            remark_duration: Duration::from_secs(8),
        }
    }

    pub fn tick(&mut self) {
        if self.last_frame_time.elapsed() >= self.frame_duration {
            self.current_frame += 1;
            self.last_frame_time = Instant::now();
        }

        // Clear expired remarks
        if let Some(start) = self.remark_start {
            if start.elapsed() >= self.remark_duration {
                self.remark_text = None;
                self.remark_start = None;
            }
        }
    }

    pub fn get_current_frame(&self, level: u8, mood: Mood) -> &'static SpriteFrame {
        let frames = get_idle_frames(level, mood);
        let idx = self.current_frame % frames.len();
        frames[idx]
    }

    pub fn set_remark(&mut self, text: String) {
        self.remark_text = Some(text);
        self.remark_start = Some(Instant::now());
    }

    pub fn current_remark(&self) -> Option<&str> {
        self.remark_text.as_deref()
    }
}

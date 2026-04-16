use std::path::Path;
use std::time::{Duration, Instant};
use crate::state::Mood;
use crate::ui::custom_sprites::{CustomSprites, DynamicSpriteFrame};
use crate::ui::sprites::{get_idle_frames, SpriteFrame};

/// Either a static (built-in) or dynamic (custom) sprite frame
pub enum SpriteRef<'a> {
    Static(&'static SpriteFrame),
    Dynamic(&'a DynamicSpriteFrame),
}

impl<'a> SpriteRef<'a> {
    pub fn lines(&self) -> Vec<&str> {
        match self {
            SpriteRef::Static(s) => s.lines.to_vec(),
            SpriteRef::Dynamic(d) => d.lines.iter().map(|s| s.as_str()).collect(),
        }
    }

    pub fn width(&self) -> u16 {
        match self {
            SpriteRef::Static(s) => s.width,
            SpriteRef::Dynamic(d) => d.width,
        }
    }

    pub fn height(&self) -> u16 {
        match self {
            SpriteRef::Static(s) => s.height,
            SpriteRef::Dynamic(d) => d.height,
        }
    }
}

pub struct AnimationState {
    current_frame: usize,
    last_frame_time: Instant,
    frame_duration: Duration,
    remark_text: Option<String>,
    remark_start: Option<Instant>,
    remark_duration: Duration,
    custom_sprites: CustomSprites,
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
            custom_sprites: CustomSprites::default(),
        }
    }

    /// Load custom sprites from the project directory
    /// Checks both .codachi/sprites/ and ascii/ folders
    pub fn load_custom_sprites(&mut self, project_dir: &Path) {
        // Try .codachi/sprites/ first
        let codachi_sprites_dir = project_dir.join(".codachi").join("sprites");
        self.custom_sprites = CustomSprites::load_from(&codachi_sprites_dir);

        // If no sprites found, try ascii/ folder
        if self.custom_sprites.is_empty() {
            let ascii_dir = project_dir.join("ascii");
            self.custom_sprites = CustomSprites::load_from(&ascii_dir);
        }
    }

    /// Reload custom sprites (call when sprite files change)
    pub fn reload_sprites(&mut self, project_dir: &Path) {
        self.load_custom_sprites(project_dir);
    }

    /// Check if custom sprites are loaded
    pub fn has_custom_sprites(&self) -> bool {
        !self.custom_sprites.is_empty()
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

    /// Get the current frame, preferring custom sprites if available
    pub fn get_current_frame(&self, level: u8, mood: Mood) -> SpriteRef<'_> {
        // Check if we have custom sprites for this level/mood
        if self.custom_sprites.has_sprites_for(level, &mood) {
            let frame_count = self.custom_sprites.frame_count(level, &mood);
            let idx = self.current_frame % frame_count;
            if let Some(frame) = self.custom_sprites.get_frame(level, &mood, idx) {
                return SpriteRef::Dynamic(frame);
            }
        }

        // Fall back to built-in sprites
        let frames = get_idle_frames(level, mood);
        let idx = self.current_frame % frames.len();
        SpriteRef::Static(frames[idx])
    }

    pub fn set_remark(&mut self, text: String) {
        self.remark_text = Some(text);
        self.remark_start = Some(Instant::now());
    }

    pub fn current_remark(&self) -> Option<&str> {
        self.remark_text.as_deref()
    }
}

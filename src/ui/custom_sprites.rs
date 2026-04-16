use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::state::Mood;

/// A dynamically loaded sprite frame (owned data, not static)
#[derive(Clone, Debug)]
pub struct DynamicSpriteFrame {
    pub lines: Vec<String>,
    pub width: u16,
    pub height: u16,
}

impl DynamicSpriteFrame {
    pub fn from_file(path: &Path) -> Option<Self> {
        let content = fs::read_to_string(path).ok()?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            return None;
        }
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as u16;
        let height = lines.len() as u16;
        Some(Self { lines, width, height })
    }
}

/// Sprite key for lookup: (level, mood, frame_index)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpriteKey {
    pub level: u8,
    pub mood: Mood,
    pub frame: usize,
}

/// Custom sprites loaded from a project's .codachi/sprites/ directory
#[derive(Default, Clone, Debug)]
pub struct CustomSprites {
    sprites: HashMap<SpriteKey, DynamicSpriteFrame>,
    frame_counts: HashMap<(u8, Mood), usize>,
}

impl CustomSprites {
    /// Load custom sprites from a directory
    /// Checks both .codachi/sprites/ and ascii/ folders
    /// Accepts multiple naming conventions:
    ///   - idle, sick, grumpy/angry, sleepy/sleep, excited/excite
    ///   - egg-idle.txt, egg-idle-2.txt, egg-idle-3.txt, etc.
    pub fn load_from(sprites_dir: &Path) -> Self {
        let mut custom = Self::default();

        if !sprites_dir.exists() || !sprites_dir.is_dir() {
            return custom;
        }

        // Define level names and their numeric values
        let levels = [("egg", 1u8), ("buddy", 2u8), ("elder", 3u8)];
        // Map moods to multiple possible file name variants
        let moods: &[(&[&str], Mood)] = &[
            (&["idle"], Mood::Happy),
            (&["sick"], Mood::Sick),
            (&["grumpy", "angry"], Mood::Grumpy),
            (&["sleepy", "sleep"], Mood::Sleepy),
            (&["excited", "excite"], Mood::Excited),
        ];

        for (level_name, level_num) in &levels {
            for (mood_names, mood) in moods {
                // Load frames for this level/mood combination
                let mut frame_idx = 0usize;

                // Try each mood name variant until we find one that exists
                let mut found_first = false;
                let mut used_mood_name = "";

                for mood_name in *mood_names {
                    let first_file = sprites_dir.join(format!("{}-{}.txt", level_name, mood_name));
                    if let Some(sprite) = DynamicSpriteFrame::from_file(&first_file) {
                        let key = SpriteKey {
                            level: *level_num,
                            mood: mood.clone(),
                            frame: frame_idx,
                        };
                        custom.sprites.insert(key, sprite);
                        frame_idx += 1;
                        found_first = true;
                        used_mood_name = mood_name;
                        break;
                    }
                }

                if found_first {
                    // Try loading additional frames (e.g., "egg-idle-2.txt", "egg-idle-3.txt", ...)
                    loop {
                        let next_file = sprites_dir.join(format!("{}-{}-{}.txt", level_name, used_mood_name, frame_idx + 1));
                        if let Some(sprite) = DynamicSpriteFrame::from_file(&next_file) {
                            let key = SpriteKey {
                                level: *level_num,
                                mood: mood.clone(),
                                frame: frame_idx,
                            };
                            custom.sprites.insert(key, sprite);
                            frame_idx += 1;
                        } else {
                            break;
                        }
                    }

                    // Record how many frames we found for this level/mood
                    if frame_idx > 0 {
                        custom.frame_counts.insert((*level_num, mood.clone()), frame_idx);
                    }
                }
            }
        }

        custom
    }

    /// Check if we have custom sprites for a given level and mood
    pub fn has_sprites_for(&self, level: u8, mood: &Mood) -> bool {
        self.frame_counts.contains_key(&(level, mood.clone()))
    }

    /// Get the number of frames for a level/mood (returns 0 if no custom sprites)
    pub fn frame_count(&self, level: u8, mood: &Mood) -> usize {
        self.frame_counts.get(&(level, mood.clone())).copied().unwrap_or(0)
    }

    /// Get a specific frame (returns None if not found)
    pub fn get_frame(&self, level: u8, mood: &Mood, frame: usize) -> Option<&DynamicSpriteFrame> {
        let key = SpriteKey {
            level,
            mood: mood.clone(),
            frame,
        };
        self.sprites.get(&key)
    }

    /// Check if any custom sprites are loaded
    pub fn is_empty(&self) -> bool {
        self.sprites.is_empty()
    }

    /// Reload sprites from directory
    pub fn reload(&mut self, sprites_dir: &Path) {
        *self = Self::load_from(sprites_dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_load_single_frame() {
        let dir = tempdir().unwrap();
        let sprites_dir = dir.path();

        fs::write(
            sprites_dir.join("egg-idle.txt"),
            "  ██  \n ████ \n  ██  \n"
        ).unwrap();

        let custom = CustomSprites::load_from(sprites_dir);

        assert!(custom.has_sprites_for(1, &Mood::Happy));
        assert_eq!(custom.frame_count(1, &Mood::Happy), 1);

        let frame = custom.get_frame(1, &Mood::Happy, 0).unwrap();
        assert_eq!(frame.lines.len(), 3);
    }

    #[test]
    fn test_load_multiple_frames() {
        let dir = tempdir().unwrap();
        let sprites_dir = dir.path();

        fs::write(sprites_dir.join("egg-idle.txt"), "frame1\n").unwrap();
        fs::write(sprites_dir.join("egg-idle-2.txt"), "frame2\n").unwrap();
        fs::write(sprites_dir.join("egg-idle-3.txt"), "frame3\n").unwrap();

        let custom = CustomSprites::load_from(sprites_dir);

        assert_eq!(custom.frame_count(1, &Mood::Happy), 3);
        assert!(custom.get_frame(1, &Mood::Happy, 0).is_some());
        assert!(custom.get_frame(1, &Mood::Happy, 1).is_some());
        assert!(custom.get_frame(1, &Mood::Happy, 2).is_some());
    }

    #[test]
    fn test_no_sprites_returns_empty() {
        let dir = tempdir().unwrap();
        let custom = CustomSprites::load_from(dir.path());

        assert!(custom.is_empty());
        assert!(!custom.has_sprites_for(1, &Mood::Happy));
    }
}

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Happy,
    Excited,
    Grumpy,
    Sleepy,
    Sick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetData {
    pub name: String,
    pub level: u8,
    pub xp: u32,
    pub health: i32,
    pub hunger: i32,
    pub cleanliness: i32,
    pub mood: Mood,
    pub alive_since: DateTime<Utc>,
}

impl Default for PetData {
    fn default() -> Self {
        Self {
            name: "Codachi".to_string(),
            level: 1,
            xp: 0,
            health: 100,
            hunger: 100,
            cleanliness: 100,
            mood: Mood::Happy,
            alive_since: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Economy {
    pub points: u32,
    pub total_commits: u32,
    pub last_known_commit_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub unlocked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub errors_since_last_save: u32,
    pub warnings_since_last_save: u32,
    pub clean_saves_streak: u32,
    pub total_errors_fixed: u32,
    pub daily_commits: u32,
    pub daily_commits_date: String,
    pub last_interaction: DateTime<Utc>,
    pub attentive_since: Option<DateTime<Utc>>,
    pub has_fed: bool,
    pub has_cleaned: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            errors_since_last_save: 0,
            warnings_since_last_save: 0,
            clean_saves_streak: 0,
            total_errors_fixed: 0,
            daily_commits: 0,
            daily_commits_date: String::new(),
            last_interaction: Utc::now(),
            attentive_since: None,
            has_fed: false,
            has_cleaned: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodachiState {
    pub pet: PetData,
    pub economy: Economy,
    pub achievements: Vec<Achievement>,
    pub session: Session,
}

impl CodachiState {
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let json = std::fs::read_to_string(path)?;
        let state: CodachiState = serde_json::from_str(&json)?;
        Ok(state)
    }
}

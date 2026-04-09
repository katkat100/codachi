use chrono::Utc;
use crate::state::{Achievement, CodachiState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AchievementId {
    ItsAlive,
    FirstCleanSave,
    OnARoll,
    Spotless,
    BugSquasher,
    Exterminator,
    FirstCommit,
    StreakOf5,
    Centurion,
    FirstMeal,
    SpaDay,
    AttentiveParent,
    Neglectful,
    GrowingUp,
    FinalForm,
    Veteran,
    OldFriends,
}

pub struct AchievementDef {
    pub id: AchievementId,
    pub key: &'static str,
    pub name: &'static str,
}

pub const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef { id: AchievementId::ItsAlive, key: "its_alive", name: "It's Alive!" },
    AchievementDef { id: AchievementId::FirstCleanSave, key: "first_clean_save", name: "First Clean Save" },
    AchievementDef { id: AchievementId::OnARoll, key: "on_a_roll", name: "On a Roll" },
    AchievementDef { id: AchievementId::Spotless, key: "spotless", name: "Spotless" },
    AchievementDef { id: AchievementId::BugSquasher, key: "bug_squasher", name: "Bug Squasher" },
    AchievementDef { id: AchievementId::Exterminator, key: "exterminator", name: "Exterminator" },
    AchievementDef { id: AchievementId::FirstCommit, key: "first_commit", name: "First Commit" },
    AchievementDef { id: AchievementId::StreakOf5, key: "streak_of_5", name: "Streak of 5" },
    AchievementDef { id: AchievementId::Centurion, key: "centurion", name: "Centurion" },
    AchievementDef { id: AchievementId::FirstMeal, key: "first_meal", name: "First Meal" },
    AchievementDef { id: AchievementId::SpaDay, key: "spa_day", name: "Spa Day" },
    AchievementDef { id: AchievementId::AttentiveParent, key: "attentive_parent", name: "Attentive Parent" },
    AchievementDef { id: AchievementId::Neglectful, key: "neglectful", name: "Neglectful" },
    AchievementDef { id: AchievementId::GrowingUp, key: "growing_up", name: "Growing Up" },
    AchievementDef { id: AchievementId::FinalForm, key: "final_form", name: "Final Form" },
    AchievementDef { id: AchievementId::Veteran, key: "veteran", name: "Veteran" },
    AchievementDef { id: AchievementId::OldFriends, key: "old_friends", name: "Old Friends" },
];

fn is_unlocked(state: &CodachiState, key: &str) -> bool {
    state.achievements.iter().any(|a| a.id == key)
}

fn check_condition(state: &CodachiState, id: AchievementId) -> bool {
    match id {
        AchievementId::ItsAlive => true,
        AchievementId::FirstCleanSave => state.session.clean_saves_streak >= 1,
        AchievementId::OnARoll => state.session.clean_saves_streak >= 5,
        AchievementId::Spotless => state.session.clean_saves_streak >= 25,
        AchievementId::BugSquasher => state.session.total_errors_fixed >= 10,
        AchievementId::Exterminator => state.session.total_errors_fixed >= 50,
        AchievementId::FirstCommit => state.economy.total_commits >= 1,
        AchievementId::StreakOf5 => state.session.daily_commits >= 5,
        AchievementId::Centurion => state.economy.total_commits >= 100,
        AchievementId::FirstMeal => state.session.has_fed,
        AchievementId::SpaDay => state.session.has_cleaned,
        AchievementId::AttentiveParent => {
            if let Some(since) = state.session.attentive_since {
                let minutes = (Utc::now() - since).num_minutes();
                state.pet.health > 80 && state.pet.hunger > 80 && state.pet.cleanliness > 80 && minutes >= 60
            } else {
                false
            }
        }
        AchievementId::Neglectful => {
            state.pet.health == 0 || state.pet.hunger == 0 || state.pet.cleanliness == 0
        }
        AchievementId::GrowingUp => state.pet.level >= 2,
        AchievementId::FinalForm => state.pet.level >= 3,
        AchievementId::Veteran => {
            let days = (Utc::now() - state.pet.alive_since).num_days();
            days >= 7
        }
        AchievementId::OldFriends => {
            let days = (Utc::now() - state.pet.alive_since).num_days();
            days >= 30
        }
    }
}

pub fn check_achievements(state: &mut CodachiState) -> Vec<AchievementId> {
    let mut newly_unlocked = Vec::new();

    for def in ACHIEVEMENTS {
        if is_unlocked(state, def.key) {
            continue;
        }
        if check_condition(state, def.id) {
            state.achievements.push(Achievement {
                id: def.key.to_string(),
                unlocked_at: Utc::now(),
            });
            newly_unlocked.push(def.id);
        }
    }

    newly_unlocked
}

pub fn get_achievement_name(id: AchievementId) -> &'static str {
    ACHIEVEMENTS.iter().find(|a| a.id == id).map(|a| a.name).unwrap_or("???")
}

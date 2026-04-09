use codachi::achievements::{check_achievements, AchievementId, ACHIEVEMENTS};
use codachi::state::CodachiState;

#[test]
fn test_first_clean_save_unlocks() {
    let mut state = CodachiState::default();
    state.session.clean_saves_streak = 1;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::FirstCleanSave));
}

#[test]
fn test_on_a_roll_unlocks_at_5_streak() {
    let mut state = CodachiState::default();
    state.session.clean_saves_streak = 5;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::OnARoll));
}

#[test]
fn test_bug_squasher_unlocks_at_10_fixes() {
    let mut state = CodachiState::default();
    state.session.total_errors_fixed = 10;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::BugSquasher));
}

#[test]
fn test_streak_of_5_daily_commits() {
    let mut state = CodachiState::default();
    state.session.daily_commits = 5;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::StreakOf5));
}

#[test]
fn test_first_meal_unlocks() {
    let mut state = CodachiState::default();
    state.session.has_fed = true;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::FirstMeal));
}

#[test]
fn test_spa_day_unlocks() {
    let mut state = CodachiState::default();
    state.session.has_cleaned = true;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::SpaDay));
}

#[test]
fn test_first_commit_unlocks() {
    let mut state = CodachiState::default();
    state.economy.total_commits = 1;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::FirstCommit));
}

#[test]
fn test_centurion_unlocks_at_100() {
    let mut state = CodachiState::default();
    state.economy.total_commits = 100;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::Centurion));
}

#[test]
fn test_growing_up_unlocks_at_level_2() {
    let mut state = CodachiState::default();
    state.pet.level = 2;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::GrowingUp));
}

#[test]
fn test_neglectful_unlocks_at_zero_stat() {
    let mut state = CodachiState::default();
    state.pet.hunger = 0;
    let newly_unlocked = check_achievements(&mut state);
    assert!(newly_unlocked.contains(&AchievementId::Neglectful));
}

#[test]
fn test_already_unlocked_achievement_not_repeated() {
    let mut state = CodachiState::default();
    state.session.clean_saves_streak = 1;
    let first = check_achievements(&mut state);
    assert!(first.contains(&AchievementId::FirstCleanSave));
    let second = check_achievements(&mut state);
    assert!(!second.contains(&AchievementId::FirstCleanSave));
}

#[test]
fn test_all_achievements_have_names() {
    for a in ACHIEVEMENTS {
        assert!(!a.name.is_empty());
    }
}

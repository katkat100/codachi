use codachi::state::{CodachiState, Mood};
use tempfile::TempDir;

#[test]
fn test_default_state_has_full_stats() {
    let state = CodachiState::default();
    assert_eq!(state.pet.level, 1);
    assert_eq!(state.pet.xp, 0);
    assert_eq!(state.pet.health, 100);
    assert_eq!(state.pet.hunger, 100);
    assert_eq!(state.pet.cleanliness, 100);
    assert_eq!(state.pet.mood, Mood::Happy);
    assert_eq!(state.economy.points, 0);
    assert_eq!(state.economy.total_commits, 0);
    assert!(state.achievements.is_empty());
    assert_eq!(state.session.total_errors_fixed, 0);
    assert_eq!(state.session.daily_commits, 0);
    assert!(!state.session.has_fed);
    assert!(!state.session.has_cleaned);
}

#[test]
fn test_state_round_trip_to_json() {
    let state = CodachiState::default();
    let json = serde_json::to_string_pretty(&state).unwrap();
    let loaded: CodachiState = serde_json::from_str(&json).unwrap();
    assert_eq!(loaded.pet.health, 100);
    assert_eq!(loaded.pet.level, 1);
    assert_eq!(loaded.economy.points, 0);
}

#[test]
fn test_save_and_load_from_disk() {
    let dir = TempDir::new().unwrap();
    let state_dir = dir.path().join(".codachi");
    std::fs::create_dir_all(&state_dir).unwrap();
    let path = state_dir.join("state.json");

    let mut state = CodachiState::default();
    state.pet.health = 42;
    state.economy.points = 7;
    state.save_to(&path).unwrap();

    let loaded = CodachiState::load_from(&path).unwrap();
    assert_eq!(loaded.pet.health, 42);
    assert_eq!(loaded.economy.points, 7);
}

#[test]
fn test_load_missing_file_returns_default() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join(".codachi/state.json");
    let loaded = CodachiState::load_from(&path).unwrap();
    assert_eq!(loaded.pet.health, 100);
}

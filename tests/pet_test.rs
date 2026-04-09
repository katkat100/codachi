use codachi::pet::{apply_save_result, calculate_mood, check_level_up, apply_feed, apply_clean, apply_hunger_decay};
use codachi::state::{CodachiState, Mood};

#[test]
fn test_clean_save_heals_and_adds_xp() {
    let mut state = CodachiState::default();
    state.pet.health = 80;
    apply_save_result(&mut state, 0, 0);
    assert_eq!(state.pet.health, 81);
    assert_eq!(state.pet.xp, 5);
    assert_eq!(state.session.clean_saves_streak, 1);
}

#[test]
fn test_errors_damage_health_and_cleanliness() {
    let mut state = CodachiState::default();
    apply_save_result(&mut state, 3, 0);
    assert_eq!(state.pet.health, 85);
    assert_eq!(state.pet.cleanliness, 91);
    assert_eq!(state.session.clean_saves_streak, 0);
}

#[test]
fn test_warnings_cause_minor_damage() {
    let mut state = CodachiState::default();
    apply_save_result(&mut state, 0, 4);
    assert_eq!(state.pet.health, 92);
    assert_eq!(state.pet.cleanliness, 96);
}

#[test]
fn test_fixing_errors_heals() {
    let mut state = CodachiState::default();
    state.session.errors_since_last_save = 5;
    state.pet.health = 60;
    apply_save_result(&mut state, 2, 0);
    assert_eq!(state.pet.health, 59);
}

#[test]
fn test_stats_clamp_to_0_100() {
    let mut state = CodachiState::default();
    state.pet.health = 5;
    apply_save_result(&mut state, 10, 0);
    assert_eq!(state.pet.health, 0);
}

#[test]
fn test_mood_happy_when_stats_good() {
    let state = CodachiState::default();
    assert_eq!(calculate_mood(&state, 0, false), Mood::Happy);
}

#[test]
fn test_mood_sick_when_health_low() {
    let mut state = CodachiState::default();
    state.pet.health = 20;
    assert_eq!(calculate_mood(&state, 0, false), Mood::Sick);
}

#[test]
fn test_mood_grumpy_when_hungry() {
    let mut state = CodachiState::default();
    state.pet.hunger = 10;
    assert_eq!(calculate_mood(&state, 0, false), Mood::Grumpy);
}

#[test]
fn test_mood_sick_overrides_grumpy() {
    let mut state = CodachiState::default();
    state.pet.health = 15;
    state.pet.hunger = 10;
    assert_eq!(calculate_mood(&state, 0, false), Mood::Sick);
}

#[test]
fn test_mood_excited_when_all_stats_high() {
    let mut state = CodachiState::default();
    state.pet.health = 90;
    state.pet.hunger = 90;
    state.pet.cleanliness = 90;
    assert_eq!(calculate_mood(&state, 0, false), Mood::Excited);
}

#[test]
fn test_mood_sleepy_after_60_min() {
    let state = CodachiState::default();
    assert_eq!(calculate_mood(&state, 60, false), Mood::Sleepy);
}

#[test]
fn test_mood_excited_override() {
    let state = CodachiState::default();
    assert_eq!(calculate_mood(&state, 0, true), Mood::Excited);
}

#[test]
fn test_mood_sick_overrides_sleepy() {
    let mut state = CodachiState::default();
    state.pet.health = 20;
    assert_eq!(calculate_mood(&state, 90, false), Mood::Sick);
}

#[test]
fn test_feed_costs_point_and_restores_hunger() {
    let mut state = CodachiState::default();
    state.pet.hunger = 50;
    state.economy.points = 3;
    let fed = apply_feed(&mut state);
    assert!(fed);
    assert_eq!(state.pet.hunger, 70);
    assert_eq!(state.economy.points, 2);
    assert_eq!(state.pet.xp, 3);
}

#[test]
fn test_feed_fails_with_no_points() {
    let mut state = CodachiState::default();
    state.economy.points = 0;
    let fed = apply_feed(&mut state);
    assert!(!fed);
}

#[test]
fn test_clean_restores_cleanliness() {
    let mut state = CodachiState::default();
    state.pet.cleanliness = 40;
    apply_clean(&mut state);
    assert_eq!(state.pet.cleanliness, 65);
    assert_eq!(state.pet.xp, 3);
}

#[test]
fn test_hunger_decay() {
    let mut state = CodachiState::default();
    apply_hunger_decay(&mut state);
    assert_eq!(state.pet.hunger, 99);
}

#[test]
fn test_level_up_at_500_xp() {
    let mut state = CodachiState::default();
    state.pet.xp = 500;
    state.pet.health = 50;
    let leveled = check_level_up(&mut state);
    assert!(leveled);
    assert_eq!(state.pet.level, 2);
}

#[test]
fn test_no_level_up_when_health_below_30() {
    let mut state = CodachiState::default();
    state.pet.xp = 500;
    state.pet.health = 20;
    let leveled = check_level_up(&mut state);
    assert!(!leveled);
    assert_eq!(state.pet.level, 1);
}

#[test]
fn test_no_level_up_past_3() {
    let mut state = CodachiState::default();
    state.pet.level = 3;
    state.pet.xp = 9999;
    let leveled = check_level_up(&mut state);
    assert!(!leveled);
    assert_eq!(state.pet.level, 3);
}

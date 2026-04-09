use crate::state::{CodachiState, Mood};

fn clamp(val: i32) -> i32 {
    val.clamp(0, 100)
}

pub fn apply_save_result(state: &mut CodachiState, errors: u32, warnings: u32) {
    let prev_errors = state.session.errors_since_last_save;

    state.pet.health -= (errors as i32) * 5;
    state.pet.health -= (warnings as i32) * 2;
    state.pet.cleanliness -= (errors as i32) * 3;
    state.pet.cleanliness -= (warnings as i32) * 1;

    if errors < prev_errors {
        let fixed = prev_errors - errors;
        state.pet.health += (fixed as i32) * 3;
        state.session.total_errors_fixed += fixed;
    }

    if errors == 0 && warnings == 0 {
        state.pet.health += 1;
        state.pet.xp += 5;
        state.session.clean_saves_streak += 1;
    } else {
        state.session.clean_saves_streak = 0;
    }

    state.pet.health = clamp(state.pet.health);
    state.pet.cleanliness = clamp(state.pet.cleanliness);
    state.session.errors_since_last_save = errors;
    state.session.warnings_since_last_save = warnings;
}

pub fn calculate_mood(state: &CodachiState, minutes_since_interaction: u64, excited_override: bool) -> Mood {
    let h = state.pet.health;
    let hu = state.pet.hunger;
    let c = state.pet.cleanliness;

    let stats_below_20 = [h, hu, c].iter().filter(|&&v| v < 20).count();

    if h < 30 || stats_below_20 >= 2 {
        return Mood::Sick;
    }
    if hu < 20 || c < 20 {
        return Mood::Grumpy;
    }
    if minutes_since_interaction >= 60 {
        return Mood::Sleepy;
    }
    if excited_override || (h > 80 && h < 100 && hu > 80 && hu < 100 && c > 80 && c < 100) {
        return Mood::Excited;
    }
    Mood::Happy
}

pub fn apply_feed(state: &mut CodachiState) -> bool {
    if state.economy.points == 0 {
        return false;
    }
    state.economy.points -= 1;
    state.pet.hunger = clamp(state.pet.hunger + 20);
    state.pet.xp += 3;
    true
}

pub fn apply_clean(state: &mut CodachiState) {
    state.pet.cleanliness = clamp(state.pet.cleanliness + 25);
    state.pet.xp += 3;
}

pub fn apply_hunger_decay(state: &mut CodachiState) {
    state.pet.hunger = clamp(state.pet.hunger - 1);
    if state.pet.hunger == 0 {
        state.pet.health = clamp(state.pet.health - 1);
    }
}

pub fn check_level_up(state: &mut CodachiState) -> bool {
    if state.pet.level >= 3 {
        return false;
    }
    if state.pet.health < 30 {
        return false;
    }
    let threshold = match state.pet.level {
        1 => 500,
        2 => 1500,
        _ => return false,
    };
    if state.pet.xp >= threshold {
        state.pet.level += 1;
        true
    } else {
        false
    }
}

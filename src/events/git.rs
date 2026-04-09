use anyhow::Result;
use crate::state::CodachiState;
use std::path::Path;
use std::process::Command;

pub fn get_commit_count(project_dir: &Path) -> Result<u32> {
    let output = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(project_dir)
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let count_str = String::from_utf8_lossy(&out.stdout);
            Ok(count_str.trim().parse().unwrap_or(0))
        }
        _ => Ok(0),
    }
}

pub fn apply_new_commits(state: &mut CodachiState, current_count: u32) -> u32 {
    if current_count <= state.economy.last_known_commit_count {
        state.economy.last_known_commit_count = current_count;
        return 0;
    }
    let new = current_count - state.economy.last_known_commit_count;
    state.economy.points += new;
    state.economy.total_commits += new;
    state.pet.xp += new * 10;
    state.economy.last_known_commit_count = current_count;
    new
}

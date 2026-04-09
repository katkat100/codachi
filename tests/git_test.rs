use codachi::events::git::{get_commit_count, apply_new_commits};
use codachi::state::CodachiState;
use tempfile::TempDir;
use std::process::Command;

fn init_git_repo(dir: &std::path::Path) {
    Command::new("git").args(["init"]).current_dir(dir).output().unwrap();
    Command::new("git").args(["config", "user.email", "test@test.com"]).current_dir(dir).output().unwrap();
    Command::new("git").args(["config", "user.name", "Test"]).current_dir(dir).output().unwrap();
}

fn make_commit(dir: &std::path::Path) {
    let file = dir.join("file.txt");
    let content = format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
    std::fs::write(&file, content).unwrap();
    Command::new("git").args(["add", "."]).current_dir(dir).output().unwrap();
    Command::new("git").args(["commit", "-m", "test"]).current_dir(dir).output().unwrap();
}

#[test]
fn test_get_commit_count_in_git_repo() {
    let dir = TempDir::new().unwrap();
    init_git_repo(dir.path());
    make_commit(dir.path());
    make_commit(dir.path());
    let count = get_commit_count(dir.path()).unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_get_commit_count_no_git_repo() {
    let dir = TempDir::new().unwrap();
    let count = get_commit_count(dir.path()).unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_apply_new_commits_awards_points() {
    let mut state = CodachiState::default();
    state.economy.last_known_commit_count = 5;
    let new_commits = apply_new_commits(&mut state, 8);
    assert_eq!(new_commits, 3);
    assert_eq!(state.economy.points, 3);
    assert_eq!(state.economy.total_commits, 3);
    assert_eq!(state.pet.xp, 30);
    assert_eq!(state.economy.last_known_commit_count, 8);
}

#[test]
fn test_apply_new_commits_no_change() {
    let mut state = CodachiState::default();
    state.economy.last_known_commit_count = 5;
    let new_commits = apply_new_commits(&mut state, 5);
    assert_eq!(new_commits, 0);
    assert_eq!(state.economy.points, 0);
}

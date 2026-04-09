use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::achievements::check_achievements;
use crate::config::Config;
use crate::events::git::{apply_new_commits, get_commit_count};
use crate::events::linter::{parse_output, run_lint, LintResult};
use crate::events::watcher::start_watcher;
use crate::pet::{apply_clean, apply_feed, apply_hunger_decay, apply_save_result, calculate_mood, check_level_up};
use crate::remarks::{get_remark, RemarkEvent};
use crate::state::CodachiState;
use crate::ui::animations::AnimationState;
use crate::ui::layout::draw;

pub struct App {
    state: CodachiState,
    config: Config,
    anim: AnimationState,
    project_dir: PathBuf,
    state_path: PathBuf,
    last_git_check: Instant,
    last_hunger_tick: Instant,
    last_xp_tick: Instant,
    last_interaction: Instant,
    last_clean_time: Option<Instant>,
    excited_until: Option<Instant>,
    show_achievements: bool,
    should_quit: bool,
}

impl App {
    pub fn new(project_dir: &Path) -> Result<Self> {
        let codachi_dir = project_dir.join(".codachi");
        let state_path = codachi_dir.join("state.json");
        let config_path = codachi_dir.join("config.toml");

        let state = CodachiState::load_from(&state_path)?;
        let config = Config::load_from(&config_path)?;

        Ok(Self {
            state,
            config,
            anim: AnimationState::new(),
            project_dir: project_dir.to_path_buf(),
            state_path,
            last_git_check: Instant::now(),
            last_hunger_tick: Instant::now(),
            last_xp_tick: Instant::now(),
            last_interaction: Instant::now(),
            last_clean_time: None,
            excited_until: None,
            show_achievements: false,
            should_quit: false,
        })
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let (fs_tx, fs_rx) = mpsc::channel();
        let _watcher = start_watcher(&self.project_dir, self.config.watch_patterns.clone(), fs_tx)?;

        // Check for initial achievements (e.g., It's Alive!)
        let initial = check_achievements(&mut self.state);
        if !initial.is_empty() {
            self.anim.set_remark("It's Alive!".to_string());
        }

        // Seed initial commit count
        if self.state.economy.last_known_commit_count == 0 {
            let count = get_commit_count(&self.project_dir).unwrap_or(0);
            self.state.economy.last_known_commit_count = count;
        }

        loop {
            // Draw
            terminal.draw(|frame| draw(frame, &self.state, &self.anim, self.show_achievements))?;

            // Handle input (non-blocking, 100ms timeout for ~10fps event checking)
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.last_interaction = Instant::now();
                        match key.code {
                            KeyCode::Char('q') => self.should_quit = true,
                            KeyCode::Char('f') => self.handle_feed(),
                            KeyCode::Char('c') => self.handle_clean(),
                            KeyCode::Char('a') => self.show_achievements = !self.show_achievements,
                            _ => {}
                        }
                    }
                }
            }

            if self.should_quit {
                self.state.save_to(&self.state_path)?;
                break;
            }

            // Animation tick
            self.anim.tick();

            // Check for file saves
            if let Ok(paths) = fs_rx.try_recv() {
                self.handle_save(&paths);
            }

            // Periodic git check (every 30s)
            if self.last_git_check.elapsed() >= Duration::from_secs(30) {
                self.handle_git_check();
                self.last_git_check = Instant::now();
            }

            // Hunger decay (every 15 min)
            if self.last_hunger_tick.elapsed() >= Duration::from_secs(900) {
                apply_hunger_decay(&mut self.state);
                if self.state.pet.hunger == 0 {
                    self.anim
                        .set_remark(get_remark(&RemarkEvent::Starving).to_string());
                }
                self.last_hunger_tick = Instant::now();
            }

            // XP from active time (every 10 min)
            if self.last_xp_tick.elapsed() >= Duration::from_secs(600) {
                self.state.pet.xp += 1;
                self.last_xp_tick = Instant::now();
            }

            // Neglect remark (30+ min no interaction)
            let mins_since_interaction = self.last_interaction.elapsed().as_secs() / 60;
            if mins_since_interaction >= 30 && mins_since_interaction % 30 == 0 {
                if self.anim.current_remark().is_none() {
                    self.anim.set_remark(get_remark(&RemarkEvent::Neglect).to_string());
                }
            }

            // Track attentive parent (all stats > 80)
            if self.state.pet.health > 80 && self.state.pet.hunger > 80 && self.state.pet.cleanliness > 80 {
                if self.state.session.attentive_since.is_none() {
                    self.state.session.attentive_since = Some(chrono::Utc::now());
                }
            } else {
                self.state.session.attentive_since = None;
            }

            // Clear excited override after 10 seconds
            let excited_override = self.excited_until.map(|t| Instant::now() < t).unwrap_or(false);

            // Update mood
            self.state.pet.mood = calculate_mood(&self.state, mins_since_interaction, excited_override);

            // Check achievements
            check_achievements(&mut self.state);
        }

        Ok(())
    }

    fn handle_save(&mut self, _paths: &[PathBuf]) {
        let output = run_lint(&self.config.lint_cmd, &self.project_dir);
        let result = match output {
            Ok(out) => parse_output(
                &out,
                &self.config.lint_parser,
                self.config.error_pattern.as_deref(),
                self.config.warning_pattern.as_deref(),
            ),
            Err(_) => LintResult::default(),
        };

        let prev_errors = self.state.session.errors_since_last_save;
        apply_save_result(&mut self.state, result.errors, result.warnings);

        // Pick remark
        let remark_event = if result.errors > 0 {
            Some(RemarkEvent::ErrorSave)
        } else if result.warnings > 0 {
            Some(RemarkEvent::WarningSave)
        } else if prev_errors > 0 && result.errors == 0 {
            Some(RemarkEvent::ErrorsFixed)
        } else {
            Some(RemarkEvent::CleanSave)
        };

        if let Some(event) = remark_event {
            self.anim.set_remark(get_remark(&event).to_string());
        }

        // Check achievements and level up
        let new_achievements = check_achievements(&mut self.state);
        if !new_achievements.is_empty() {
            self.excited_until = Some(Instant::now() + Duration::from_secs(10));
        }
        if check_level_up(&mut self.state) {
            self.anim
                .set_remark(get_remark(&RemarkEvent::LevelUp).to_string());
            self.excited_until = Some(Instant::now() + Duration::from_secs(10));
        }

        self.state.save_to(&self.state_path).ok();
    }

    fn handle_feed(&mut self) {
        if apply_feed(&mut self.state) {
            self.state.session.has_fed = true;
            self.anim
                .set_remark(get_remark(&RemarkEvent::Feeding).to_string());
            check_achievements(&mut self.state);
            if check_level_up(&mut self.state) {
                self.anim
                    .set_remark(get_remark(&RemarkEvent::LevelUp).to_string());
                self.excited_until = Some(Instant::now() + Duration::from_secs(10));
            }
            self.state.save_to(&self.state_path).ok();
        }
    }

    fn handle_clean(&mut self) {
        if let Some(last) = self.last_clean_time {
            if last.elapsed() < Duration::from_secs(300) {
                return; // cooldown
            }
        }
        apply_clean(&mut self.state);
        self.state.session.has_cleaned = true;
        self.last_clean_time = Some(Instant::now());
        self.anim
            .set_remark(get_remark(&RemarkEvent::Cleaning).to_string());
        check_achievements(&mut self.state);
        if check_level_up(&mut self.state) {
            self.anim
                .set_remark(get_remark(&RemarkEvent::LevelUp).to_string());
            self.excited_until = Some(Instant::now() + Duration::from_secs(10));
        }
        self.state.save_to(&self.state_path).ok();
    }

    fn handle_git_check(&mut self) {
        if let Ok(count) = get_commit_count(&self.project_dir) {
            let new = apply_new_commits(&mut self.state, count);
            if new > 0 {
                // Track daily commits
                let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
                if self.state.session.daily_commits_date != today {
                    self.state.session.daily_commits = 0;
                    self.state.session.daily_commits_date = today;
                }
                self.state.session.daily_commits += new;

                check_achievements(&mut self.state);
                self.state.save_to(&self.state_path).ok();
            }
        }
    }
}

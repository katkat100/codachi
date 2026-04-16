use anyhow::Result;
use glob_match::glob_match;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

/// Events from the file watcher
#[derive(Debug, Clone)]
pub enum WatchEvent {
    /// Source files changed (triggers lint)
    SourceChanged(Vec<std::path::PathBuf>),
    /// Sprite files changed (triggers reload)
    SpritesChanged,
}

pub fn start_watcher(
    project_dir: &Path,
    watch_patterns: Vec<String>,
    tx: mpsc::Sender<WatchEvent>,
) -> Result<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(500), notify_tx)?;

    debouncer
        .watcher()
        .watch(project_dir, notify::RecursiveMode::Recursive)?;

    let project_dir = project_dir.to_path_buf();
    std::thread::spawn(move || {
        while let Ok(Ok(events)) = notify_rx.recv() {
            let all_paths: Vec<std::path::PathBuf> = events
                .into_iter()
                .filter(|e| e.kind == DebouncedEventKind::Any)
                .map(|e| e.path)
                .collect();

            // Check if any sprite files changed (in .codachi/sprites/ or ascii/)
            let sprites_changed = all_paths.iter().any(|p| {
                let rel = p.strip_prefix(&project_dir).unwrap_or(p);
                let rel_str = rel.to_string_lossy();
                (rel_str.starts_with(".codachi/sprites/") || rel_str.starts_with("ascii/"))
                    && rel_str.ends_with(".txt")
            });

            if sprites_changed {
                let _ = tx.send(WatchEvent::SpritesChanged);
            }

            // Filter for source file changes
            let source_paths: Vec<std::path::PathBuf> = all_paths
                .into_iter()
                .filter(|p| {
                    let rel = p.strip_prefix(&project_dir).unwrap_or(p);
                    let is_hidden = rel.components().any(|c| {
                        c.as_os_str()
                            .to_str()
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                    });
                    if is_hidden {
                        return false;
                    }
                    let rel_str = rel.to_string_lossy();
                    watch_patterns.iter().any(|pat| glob_match(pat, &rel_str))
                })
                .collect();

            if !source_paths.is_empty() {
                let _ = tx.send(WatchEvent::SourceChanged(source_paths));
            }
        }
    });

    Ok(debouncer)
}

use anyhow::Result;
use glob_match::glob_match;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

pub fn start_watcher(
    project_dir: &Path,
    watch_patterns: Vec<String>,
    tx: mpsc::Sender<Vec<std::path::PathBuf>>,
) -> Result<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(500), notify_tx)?;

    debouncer
        .watcher()
        .watch(project_dir, notify::RecursiveMode::Recursive)?;

    let project_dir = project_dir.to_path_buf();
    std::thread::spawn(move || {
        while let Ok(Ok(events)) = notify_rx.recv() {
            let paths: Vec<std::path::PathBuf> = events
                .into_iter()
                .filter(|e| e.kind == DebouncedEventKind::Any)
                .map(|e| e.path)
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

            if !paths.is_empty() {
                let _ = tx.send(paths);
            }
        }
    });

    Ok(debouncer)
}

use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::{Config, EventKind, PollWatcher, RecursiveMode, Watcher};

use crate::config::read_universal_config;
use crate::detect::all_renderers;

pub fn run(project_root: &Path) -> Result<()> {
    let source = project_root.join(".agents").join("mcp.json");
    let pid_path = pid_file_path();

    if let Some(parent) = pid_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&pid_path, format!("{}", std::process::id()))?;

    let (tx, rx) = mpsc::channel();
    let mut watcher = PollWatcher::new(tx, Config::default())
        .context("Failed to create file watcher")?;
    watcher
        .watch(&source, RecursiveMode::NonRecursive)
        .with_context(|| format!("Failed to watch {}", source.display()))?;

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if is_write_event(&event) {
                    thread::sleep(Duration::from_millis(200));
                    if let Ok(config) = read_universal_config(&source) {
                        for renderer in all_renderers() {
                            if let Err(e) = renderer.write(&config, project_root) {
                                eprintln!("[agents-mcp] {} write error: {e}", renderer.name());
                            }
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("[agents-mcp] watch error: {e}"),
            Err(mpsc::RecvError) => break,
        }
    }

    let _ = fs::remove_file(&pid_path);
    Ok(())
}

fn is_write_event(event: &notify::Event) -> bool {
    matches!(
        event.kind,
        EventKind::Modify(_) | EventKind::Create(_)
    )
}

pub fn pid_file_path() -> std::path::PathBuf {
    let base = dirs::data_local_dir()
        .unwrap_or_else(|| Path::new("/tmp").to_path_buf())
        .join("agents-mcp");
    base.join("daemon.pid")
}

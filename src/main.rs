mod cli;
mod config;
mod daemon;
mod detect;
mod renderers;
mod service;
mod sync;

use anyhow::Result;
use clap::Parser;
use cli::{AgentsAction, Cli, Command};

fn find_project_root() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir()?;
    let mut dir = Some(cwd.as_path());
    while let Some(d) = dir {
        if d.join(".agents").join("mcp.json").exists() {
            return Ok(d.to_path_buf());
        }
        dir = d.parent();
    }
    anyhow::bail!(
        "No .agents/mcp.json found in current or parent directories.\n\
         Create one at the root of your project."
    );
}

fn status() -> Result<()> {
    let pid_path = daemon::pid_file_path();
    let pid = pid_path.exists().then(|| {
        std::fs::read_to_string(&pid_path)
            .ok()
            .and_then(|s| s.trim().parse::<u32>().ok())
    }).flatten();

    let alive = pid.is_some_and(|pid| {
        std::path::Path::new(&format!("/proc/{}", pid)).exists()
    });

    if alive {
        println!("● Daemon running (PID {})", pid.unwrap());
    } else {
        println!("○ Daemon not running");
    }

    let project_root = find_project_root()?;
    let agents = detect::detect_agents(&project_root);
    println!("\nDetected agents:");
    for agent in &agents {
        let icon = if agent.present { "✓" } else { " " };
        println!("  {} {}", icon, agent.name);
    }

    Ok(())
}

fn sync() -> Result<()> {
    let project_root = find_project_root()?;
    sync::sync_all(&project_root)?;
    println!("✓ All agents synced");
    Ok(())
}

fn stop() -> Result<()> {
    let pid_path = daemon::pid_file_path();
    if pid_path.exists() {
        service::stop()?;
        let _ = std::fs::remove_file(&pid_path);
        println!("✓ Daemon stopped");
    } else {
        println!("○ Daemon not running");
    }
    Ok(())
}

fn agents_list() -> Result<()> {
    let agents = detect::detect_os_agents();
    println!("Agents installed on this system:\n");
    for a in &agents {
        let icon = if a.installed { "✓" } else { " " };
        print!("  {} {}", icon, a.name);
        if let Some(note) = &a.note {
            print!(" ({})", note);
        }
        println!();
        if let Some(loc) = &a.location {
            println!("       {}", loc);
        }
    }
    Ok(())
}

fn uninstall() -> Result<()> {
    stop()?;
    service::uninstall()?;
    println!("✓ Service uninstalled");
    Ok(())
}

fn first_run() -> Result<()> {
    let project_root = find_project_root()?;

    if !service::is_installed() {
        println!("Installing service...");
        if let Err(e) = service::install() {
            let is_perm = e.chain().any(|cause| {
                cause.downcast_ref::<std::io::Error>()
                    .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
            });
            if is_perm {
                anyhow::bail!(
                    "Permission denied. Run with sudo:\n  sudo mcplink"
                );
            }
            return Err(e);
        }
    } else {
        println!("Service already installed, starting...");
        service::start()?;
    }

    let agents = detect::detect_agents(&project_root);
    println!("\nmcplink v{}", env!("CARGO_PKG_VERSION"));
    println!("Universal MCP config sync daemon\n");
    println!("✓ Service installed");
    println!("✓ Auto-start enabled");
    println!("✓ Daemon running");
    println!("✓ Watching .agents/mcp.json\n");
    println!("Detected agents: {}", 
        agents.iter()
            .filter(|a| a.present)
            .map(|a| a.name)
            .collect::<Vec<_>>()
            .join(", ")
    );
    if agents.iter().all(|a| !a.present) {
        println!("  (none detected — will create configs on first sync)");
    }

    // Run initial sync
    sync::sync_all(&project_root)?;
    println!("\nEdit .agents/mcp.json once. All agents stay in sync.");

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.daemon {
        let project_root = find_project_root()?;
        daemon::run(&project_root)
    } else {
        match cli.command {
            Some(Command::Status) => status(),
            Some(Command::Sync) => sync(),
            Some(Command::Stop) => stop(),
            Some(Command::Uninstall) => uninstall(),
            Some(Command::Agents { action }) => {
                match action.unwrap_or(AgentsAction::List) {
                    AgentsAction::List => agents_list(),
                }
            }
            None => first_run(),
        }
    }
}

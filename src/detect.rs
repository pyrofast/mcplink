use std::path::Path;

use crate::renderers::claude::ClaudeRenderer;
use crate::renderers::copilot::CopilotRenderer;
use crate::renderers::cursor::CursorRenderer;
use crate::renderers::opencode::OpenCodeRenderer;
use crate::renderers::vscode::VSCodeRenderer;
use crate::renderers::windsurf::WindsurfRenderer;
use crate::renderers::AgentRenderer;

pub struct AgentInfo {
    pub name: &'static str,
    pub present: bool,
}

pub fn all_renderers() -> Vec<Box<dyn AgentRenderer>> {
    vec![
        Box::new(CursorRenderer),
        Box::new(ClaudeRenderer),
        Box::new(CopilotRenderer),
        Box::new(VSCodeRenderer),
        Box::new(WindsurfRenderer),
        Box::new(OpenCodeRenderer),
    ]
}

pub fn detect_agents(project_root: &Path) -> Vec<AgentInfo> {
    all_renderers().into_iter().map(|r| AgentInfo {
        name: r.name(),
        present: r.exists(project_root),
    }).collect()
}

#[derive(Debug)]
pub struct OsAgentInfo {
    pub name: &'static str,
    pub installed: bool,
    pub location: Option<String>,
    pub note: Option<&'static str>,
}

fn find_in_path(binary: &str) -> Option<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':').find_map(|dir| {
        let full = Path::new(dir).join(binary);
        if full.exists() { Some(full.to_string_lossy().to_string()) } else { None }
    })
}

fn home_dir() -> std::path::PathBuf {
    std::env::var("HOME").map(PathBuf::from).unwrap_or_default()
}

pub fn detect_os_agents() -> Vec<OsAgentInfo> {
    let mut agents = Vec::new();

    // Cursor
    {
        let loc = find_in_path("cursor")
            .or_else(|| {
                let p = home_dir().join(".cursor").join("bin").join("cursor");
                if p.exists() { Some(p.to_string_lossy().to_string()) } else { None }
            })
            .or_else(|| {
                let p = Path::new("/Applications/Cursor.app");
                if p.exists() { Some(p.to_string_lossy().to_string()) } else { None }
            });
        agents.push(OsAgentInfo {
            name: "Cursor",
            installed: loc.is_some(),
            location: loc,
            note: None,
        });
    }

    // Claude Code
    {
        let loc = find_in_path("claude")
            .or_else(|| {
                let p = home_dir().join(".local").join("bin").join("claude");
                if p.exists() { Some(p.to_string_lossy().to_string()) } else { None }
            });
        let has_config = home_dir().join(".claude.json").exists();
        agents.push(OsAgentInfo {
            name: "Claude Code",
            installed: loc.is_some() || has_config,
            location: loc,
            note: if !loc.is_some() && has_config { Some("config found (~/.claude.json)") } else { None },
        });
    }

    // Copilot
    {
        let loc = find_in_path("gh")
            .map(|p| format!("{} (gh CLI)", p));
        agents.push(OsAgentInfo {
            name: "Copilot",
            installed: loc.is_some(),
            location: loc,
            note: if loc.is_some() { Some("via GitHub CLI") } else { None },
        });
    }

    // VS Code
    {
        let loc = find_in_path("code")
            .or_else(|| {
                let p = Path::new("/Applications/Visual Studio Code.app");
                if p.exists() { Some(p.to_string_lossy().to_string()) } else { None }
            });
        agents.push(OsAgentInfo {
            name: "VS Code",
            installed: loc.is_some(),
            location: loc,
            note: None,
        });
    }

    // Devin Desktop (formerly Windsurf)
    {
        let loc = find_in_path("devin-desktop")
            .or_else(|| find_in_path("devin"))
            .or_else(|| find_in_path("windsurf"))
            .or_else(|| find_in_path("surf"))
            .or_else(|| {
                let config_new = home_dir().join(".config").join("Devin");
                let config_old = home_dir().join(".config").join("Windsurf");
                if config_new.exists() {
                    Some(config_new.to_string_lossy().to_string())
                } else if config_old.exists() {
                    Some(config_old.to_string_lossy().to_string())
                } else {
                    None
                }
            });
        agents.push(OsAgentInfo {
            name: "Devin Desktop",
            installed: loc.is_some(),
            location: loc,
            note: Some("formerly Windsurf"),
        });
    }

    // OpenCode
    {
        let loc = find_in_path("opencode");
        agents.push(OsAgentInfo {
            name: "OpenCode",
            installed: loc.is_some(),
            location: loc,
            note: None,
        });
    }

    agents
}

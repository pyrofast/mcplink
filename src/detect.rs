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

fn which(binary: &str) -> bool {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(':').any(|dir| std::path::Path::new(dir).join(binary).exists())
}

#[derive(Debug)]
pub struct OsAgentInfo {
    pub name: &'static str,
    pub binary: &'static str,
    pub installed: bool,
    pub location: Option<String>,
}

pub fn detect_os_agents() -> Vec<OsAgentInfo> {
    let agents: &[(&str, &str)] = &[
        ("cursor", "cursor"),
        ("claude-code", "claude"),
        ("copilot", "github-copilot"),
        ("vscode", "code"),
        ("windsurf", "windsurf"),
        ("opencode", "opencode"),
    ];

    let path = std::env::var("PATH").unwrap_or_default();

    agents.iter().map(|&(name, binary)| {
        let location = path.split(':').find_map(|dir| {
            let full = std::path::Path::new(dir).join(binary);
            if full.exists() { Some(full.to_string_lossy().to_string()) } else { None }
        });
        OsAgentInfo {
            name,
            binary,
            installed: location.is_some(),
            location,
        }
    }).collect()
}

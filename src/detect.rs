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
    let mut info = Vec::new();
    for r in all_renderers() {
        info.push(AgentInfo {
            name: r.name(),
            present: r.exists(project_root),
        });
    }
    info
}

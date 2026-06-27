use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use super::AgentRenderer;
use crate::config::UniversalConfig;

pub struct ClaudeRenderer;

impl AgentRenderer for ClaudeRenderer {
    fn name(&self) -> &'static str {
        "claude-code"
    }

    fn config_path(&self, project_root: &Path) -> PathBuf {
        project_root.join(".mcp.json")
    }

    fn render(&self, config: &UniversalConfig) -> Value {
        let mut servers = Map::new();
        for (name, server) in &config.servers {
            servers.insert(name.clone(), super::transform_server(server));
        }
        let mut root = Map::new();
        root.insert("mcpServers".into(), Value::Object(servers));
        Value::Object(root)
    }
}

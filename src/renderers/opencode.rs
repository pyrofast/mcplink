use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::{Map, Value};

use super::AgentRenderer;
use crate::config::{ServerConfig, Transport, UniversalConfig};

pub struct OpenCodeRenderer;

impl AgentRenderer for OpenCodeRenderer {
    fn name(&self) -> &'static str {
        "opencode"
    }

    fn config_path(&self, project_root: &Path) -> PathBuf {
        project_root.join("opencode.json")
    }

    fn render(&self, config: &UniversalConfig) -> Value {
        let mut servers = Map::new();
        for (name, server) in &config.servers {
            servers.insert(name.clone(), render_opencode_server(server));
        }
        let mut root = Map::new();
        root.insert("mcp".into(), Value::Object(servers));
        Value::Object(root)
    }

    fn write(&self, config: &UniversalConfig, project_root: &Path) -> Result<()> {
        let path = self.config_path(project_root);
        let rendered_mcp = self.render(config);

        let mut existing: Value = if path.exists() {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            serde_json::from_str(&content)
                .unwrap_or(Value::Object(Map::new()))
        } else {
            Value::Object(Map::new())
        };

        if let Value::Object(ref mut map) = existing {
            if let Some(mcp_value) = rendered_mcp.as_object().and_then(|m| m.get("mcp")) {
                map.insert("mcp".into(), mcp_value.clone());
            }
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create dirs for {}", path.display()))?;
        }
        let json = serde_json::to_string_pretty(&existing)
            .context("Failed to serialize opencode config")?;
        fs::write(&path, &json)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(())
    }
}

fn render_opencode_server(server: &ServerConfig) -> Value {
    match server.transport {
        Transport::Http => {
            let mut obj = Map::new();
            obj.insert("type".into(), Value::String("remote".into()));
            obj.insert("enabled".into(), Value::Bool(true));
            if let Some(url) = &server.url {
                obj.insert("url".into(), Value::String(url.clone()));
            }
            if let Some(headers) = &server.headers {
                obj.insert("headers".into(), serde_json::to_value(headers).unwrap_or_default());
            }
            Value::Object(obj)
        }
        Transport::Stdio => {
            let mut obj = Map::new();
            obj.insert("type".into(), Value::String("local".into()));
            obj.insert("enabled".into(), Value::Bool(true));

            let mut command = Vec::new();
            if let Some(cmd) = &server.command {
                command.push(cmd.clone());
            }
            if let Some(args) = &server.args {
                command.extend(args.clone());
            }
            obj.insert("command".into(), serde_json::to_value(command).unwrap_or_default());

            if let Some(env) = &server.env {
                obj.insert("environment".into(), serde_json::to_value(env).unwrap_or_default());
            }

            Value::Object(obj)
        }
    }
}

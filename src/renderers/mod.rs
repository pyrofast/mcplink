use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde_json::Value;

use crate::config::UniversalConfig;

pub mod claude;
pub mod copilot;
pub mod cursor;
pub mod opencode;
pub mod vscode;
pub mod windsurf;

pub trait AgentRenderer: Send + Sync {
    fn name(&self) -> &str;
    fn config_path(&self, project_root: &Path) -> PathBuf;

    fn render(&self, config: &UniversalConfig) -> Value;

    fn write(&self, config: &UniversalConfig, project_root: &Path) -> Result<()> {
        let rendered = self.render(config);
        let path = self.config_path(project_root);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create dirs for {}", path.display()))?;
        }
        let json = serde_json::to_string_pretty(&rendered)
            .context("Failed to serialize config")?;
        fs::write(&path, &json)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(())
    }

    fn exists(&self, project_root: &Path) -> bool {
        self.config_path(project_root).exists()
    }
}

fn transform_server(server: &crate::config::ServerConfig) -> Value {
    match server.transport {
        crate::config::Transport::Http => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".into(), Value::String("http".into()));
            if let Some(url) = &server.url {
                obj.insert("url".into(), Value::String(url.clone()));
            }
            if let Some(headers) = &server.headers {
                obj.insert("headers".into(), serde_json::to_value(headers).unwrap_or_default());
            }
            Value::Object(obj)
        }
        crate::config::Transport::Stdio => {
            let mut obj = serde_json::Map::new();
            if let Some(cmd) = &server.command {
                obj.insert("command".into(), Value::String(cmd.clone()));
            }
            if let Some(args) = &server.args {
                obj.insert("args".into(), serde_json::to_value(args).unwrap_or_default());
            }
            if let Some(env) = &server.env {
                obj.insert("env".into(), serde_json::to_value(env).unwrap_or_default());
            }
            Value::Object(obj)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ServerConfig, Transport};

    fn http_server() -> ServerConfig {
        ServerConfig {
            transport: Transport::Http,
            url: Some("http://localhost:3000".into()),
            headers: None,
            command: None,
            args: None,
            env: None,
        }
    }

    fn stdio_server() -> ServerConfig {
        ServerConfig {
            transport: Transport::Stdio,
            url: None,
            headers: None,
            command: Some("npx".into()),
            args: Some(vec!["-y".into(), "@mcp/server".into()]),
            env: Some([("KEY".into(), "val".into())].into()),
        }
    }

    #[test]
    fn test_transform_http() {
        let v = transform_server(&http_server());
        let obj = v.as_object().unwrap();
        assert_eq!(obj["type"], "http");
        assert_eq!(obj["url"], "http://localhost:3000");
    }

    #[test]
    fn test_transform_stdio() {
        let v = transform_server(&stdio_server());
        let obj = v.as_object().unwrap();
        assert_eq!(obj["command"], "npx");
        assert_eq!(obj["args"], serde_json::json!(["-y", "@mcp/server"]));
        assert_eq!(obj["env"], serde_json::json!({"KEY": "val"}));
    }

    #[test]
    fn test_transform_stdio_no_env() {
        let s = ServerConfig {
            env: None,
            ..stdio_server()
        };
        let v = transform_server(&s);
        let obj = v.as_object().unwrap();
        assert_eq!(obj["command"], "npx");
        assert!(obj.get("env").is_none());
    }
}

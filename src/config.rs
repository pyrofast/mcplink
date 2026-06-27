use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UniversalConfig {
    pub servers: HashMap<String, ServerConfig>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Transport {
    Http,
    Stdio,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub transport: Transport,
    pub url: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub env: Option<HashMap<String, String>>,
}

pub fn read_universal_config(path: &Path) -> Result<UniversalConfig> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    let config: UniversalConfig = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    for (name, server) in &config.servers {
        match server.transport {
            Transport::Http => {
                if server.url.is_none() {
                    anyhow::bail!("Server '{name}': http transport requires 'url'");
                }
            }
            Transport::Stdio => {
                if server.command.is_none() {
                    anyhow::bail!("Server '{name}': stdio transport requires 'command'");
                }
            }
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_tmp_config(json: &str) -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mcp.json");
        std::fs::write(&path, json).unwrap();
        (dir, path)
    }

    #[test]
    fn test_parse_valid_http_server() {
        let json = r#"{"servers":{"foo":{"transport":"http","url":"http://localhost:3000"}}}"#;
        let (_dir, path) = write_tmp_config(json);
        let config = read_universal_config(&path).unwrap();
        assert_eq!(config.servers.len(), 1);
        let s = &config.servers["foo"];
        assert!(matches!(s.transport, Transport::Http));
        assert_eq!(s.url.as_deref(), Some("http://localhost:3000"));
    }

    #[test]
    fn test_parse_valid_stdio_server() {
        let json = r#"{"servers":{"bar":{"transport":"stdio","command":"npx","args":["-y","@mcp/server"]}}}"#;
        let (_dir, path) = write_tmp_config(json);
        let config = read_universal_config(&path).unwrap();
        let s = &config.servers["bar"];
        assert!(matches!(s.transport, Transport::Stdio));
        assert_eq!(s.command.as_deref(), Some("npx"));
    }

    #[test]
    fn test_validate_http_missing_url() {
        let json = r#"{"servers":{"bad":{"transport":"http"}}}"#;
        let (_dir, path) = write_tmp_config(json);
        let result = read_universal_config(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires 'url'"));
    }

    #[test]
    fn test_validate_stdio_missing_command() {
        let json = r#"{"servers":{"bad":{"transport":"stdio"}}}"#;
        let (_dir, path) = write_tmp_config(json);
        let result = read_universal_config(&path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires 'command'"));
    }
}

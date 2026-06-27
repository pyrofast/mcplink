# mcplink

Universal MCP config sync daemon. One source of truth, all agents stay in sync.

## Problem

Every AI agent stores MCP config in its own path with its own schema. Add a server once → edit 6 files.

## Solution

`.agents/mcp.json` is the single source of truth. `mcplink` watches it and propagates every change to all agents automatically.

## Supported agents

| Agent | Config path | Key |
|---|---|---|
| Cursor | `.cursor/mcp.json` | `mcpServers` |
| Claude Code | `.mcp.json` | `mcpServers` |
| Copilot | `.github/mcp.json` | `mcpServers` |
| VS Code | `.vscode/mcp.json` | `servers` |
| Windsurf | `.windsurf/mcp.json` | `mcpServers` |
| OpenCode | `opencode.json` | `mcp` |

## Usage

```bash
# First run — installs as system service
mcplink

# Commands
mcplink status    # Show daemon status and synced agents
mcplink sync      # Force sync now
mcplink stop      # Stop the daemon
mcplink uninstall # Remove service
```

## Source of truth

Create `.agents/mcp.json` in your project root:

```json
{
  "servers": {
    "memlink": {
      "transport": "http",
      "url": "https://memlink.cloud/mcp",
      "headers": {
        "Authorization": "Bearer TOKEN"
      }
    },
    "filesystem": {
      "transport": "stdio",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/user"]
    }
  }
}
```

## Install

Download and run the binary for your platform — it auto-installs as a system daemon on first run.

### Linux (x86_64)
```bash
curl -fsSLo mcplink https://github.com/pyrofast/mcplink/releases/download/v0.1.1/mcplink-x86_64-unknown-linux-gnu
chmod +x mcplink
sudo mv mcplink /usr/local/bin/
mcplink
```

### macOS (Intel)
```bash
curl -fsSLo mcplink https://github.com/pyrofast/mcplink/releases/download/v0.1.1/mcplink-x86_64-apple-darwin
chmod +x mcplink
sudo mv mcplink /usr/local/bin/
mcplink
```

### macOS (Apple Silicon)
```bash
curl -fsSLo mcplink https://github.com/pyrofast/mcplink/releases/download/v0.1.1/mcplink-aarch64-apple-darwin
chmod +x mcplink
sudo mv mcplink /usr/local/bin/
mcplink
```

### Windows (x86_64 PowerShell as Admin)
```powershell
curl.exe -fsSLo mcplink.exe https://github.com/pyrofast/mcplink/releases/download/v0.1.1/mcplink-x86_64-pc-windows-msvc.exe
.\mcplink.exe
```

All platforms: download the right binary from [Releases](https://github.com/pyrofast/mcplink/releases), place it in your PATH, then run `mcplink`.

## Build from source

```bash
cargo build --release
```

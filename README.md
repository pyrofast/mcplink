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

Download the binary for your platform from [Releases](https://github.com/pyrofast/agents-mcp/releases), run it once, done.

## Build from source

```bash
cargo build --release
```

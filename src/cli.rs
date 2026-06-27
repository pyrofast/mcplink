use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcplink", version, about = "Universal MCP config sync daemon")]
pub struct Cli {
    #[arg(long, help = "Run as daemon (called by service)")]
    pub daemon: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(about = "Show daemon status and synced agents")]
    Status,
    #[command(about = "Force sync now")]
    Sync,
    #[command(about = "Stop the daemon")]
    Stop,
    #[command(about = "Remove service and restore original configs")]
    Uninstall,
    #[command(about = "Detect and list agents installed on this system", aliases = &["als"])]
    Agents {
        #[command(subcommand)]
        action: Option<AgentsAction>,
    },
}

#[derive(Subcommand)]
pub enum AgentsAction {
    #[command(name = "list", about = "List all available agents and their install status")]
    List,
}

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
}

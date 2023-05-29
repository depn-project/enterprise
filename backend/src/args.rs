use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    User(UserCommands),

    #[command(subcommand)]
    Server(ServerCommands),
    Run,
}

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    Create { username: String, password: String },
    Remove { username: String },
    Verify { username: String, password: String },
    List,
}

#[derive(Debug, Subcommand)]
pub enum ServerCommands {
    Create {
        country: String,
        city: String,
        vpn_config: String,
        ip: String,
        port: u16,
    },
    Remove {
        id: u32,
    },
    List,
    Config {
        id: u32,
    },
}

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(subcommand)]
    User(UserCommands),

    #[command(subcommand)]
    Server(ServerCommands),
    Run,
}

#[derive(Debug, Subcommand)]
enum UserCommands {
    Create { username: String, password: String },
    Remove { username: String },
    Verify { username: String, password: String },
    List,
}

#[derive(Debug, Subcommand)]
enum ServerCommands {
    Create { username: String, password: String },
    Remove { username: String },
    List,
}

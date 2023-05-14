use clap::{Parser, ValueEnum};
use jsonrpc_core::serde::Serialize;

#[derive(Parser)]
#[command(version)]
pub struct DepnCoreArgs {
    /// Core launch mode
    #[clap(short, long, value_enum)]
    pub mode: Mode,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum Mode {
    #[cfg(not(target_family = "windows"))]
    Gateway,
    Client,
}

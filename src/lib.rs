pub mod commands;
pub mod core;
pub mod ui;
pub mod storage;

use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ProcessCommands {
    /// Top resource-consuming processes
    Top,
    /// Find zombie processes
    Zombie,
    /// Find unusually heavy processes
    Heavy,
}

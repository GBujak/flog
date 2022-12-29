use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: ArgsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ArgsSubcommand {
    /// Add directory to git branch sources
    AddDir { dir_name: String },
    /// Remove directory from git branch sources
    RmDir { dir_name: String },
    /// Set branch format. Branch name will be split on separator and
    /// index-th element will be used for worklog tag
    SetBranchFormat { separator: String, index: usize },
    /// Set the prefix that will be used for every worklog tag
    SetTagPrefix { prefix: String },
    /// Show current configuration
    PrintConfig,
    /// Create new work log
    Log,
}

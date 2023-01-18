mod cli;
mod configuration;
mod log;
mod repo;

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use clap::Parser;
use itertools::Itertools;
use log::inquire_log;

fn main() -> Result<()> {
    let mut config = match configuration::load_config() {
        Ok(it) => it,
        Err(_) => configuration::Configuration::default(),
    };
    let args = cli::Args::parse();

    match args.subcommand {
        cli::ArgsSubcommand::AddDir { dir_name } => {
            if !config.repo_dirs.contains(&dir_name) {
                config.repo_dirs.push(dir_name);
            }
        }
        cli::ArgsSubcommand::RmDir { dir_name } => {
            config.repo_dirs.retain(|it| it != &dir_name);
        }
        cli::ArgsSubcommand::SetBranchFormat { separator, index } => {
            config.tag_configuration.separator = separator;
            config.tag_configuration.element_index = index;
        }
        cli::ArgsSubcommand::SetTagPrefix { prefix } => {
            config.tag_configuration.prefix = prefix;
        }
        cli::ArgsSubcommand::PrintConfig => println!("{}", serde_json::to_string_pretty(&config)?),
        cli::ArgsSubcommand::Log => {
            if config.repo_dirs.len() == 0 {
                return Err(anyhow!("Must set at least one repo dir before logging!"));
            }
            let branches = repo::branch_collecting::collect_branches(&config.repo_dirs)?;
            let log = inquire_log(&config, &branches)?;
            let log_json = serde_json::to_string_pretty(
                &log.into_iter().map(|it| it.to_serializable()).collect_vec(),
            )?;

            Clipboard::new()?.set_text(&log_json)?;
            println!("Result:\n\n{log_json}\n\nCopied to clipboard!");
        }
    };

    configuration::save_config(config)?;
    Ok(())
}

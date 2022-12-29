use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &'static str = "flog-config.json";

#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub repo_dirs: Vec<String>,
    pub default_project: String,
    pub tag_configuration: TagConfiguration,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            repo_dirs: vec![],
            default_project: "PROJ".to_string(),
            tag_configuration: TagConfiguration {
                separator: "/".to_string(),
                element_index: 1,
                prefix: "#CW".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagConfiguration {
    pub separator: String,
    pub element_index: usize,
    pub prefix: String,
}

pub fn load_config() -> Result<Configuration> {
    let file_name = get_config_filename()?;
    let file = File::open(file_name)?;
    Ok(serde_json::from_reader(BufReader::new(file))?)
}

pub fn save_config(config: Configuration) -> Result<()> {
    let file_name = get_config_filename()?;
    let file = File::create(file_name)?;
    serde_json::to_writer_pretty(BufWriter::new(file), &config)?;
    Ok(())
}

fn get_config_filename() -> Result<PathBuf> {
    let project_dirs = ProjectDirs::from("github", "gbujak", "flog")
        .ok_or(anyhow!("could not get config directory"))?;
    let config_dir = project_dirs.config_dir();
    create_dir_all(config_dir)?;

    let mut buf = config_dir.to_owned();
    buf.push(CONFIG_FILE_NAME);
    Ok(buf)
}

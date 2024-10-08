use std::sync::OnceLock;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::pikpak::ClientOptions;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub proxy: Option<String>,
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub log_path: String,
}

static CONF: OnceLock<Config> = OnceLock::new();

pub fn load_config(path: &str) -> Result<()> {
    let file = std::fs::File::open(path).context("load file failed")?;

    let conf: Config = serde_yaml::from_reader(file).context("parse file failed")?;
    CONF.set(conf)
        .map_err(|_| anyhow::anyhow!("failed to set config"))?;

    Ok(())
}

pub fn get_config() -> &'static Config {
    CONF.get().expect("config not init")
}

pub fn get_client_options() -> ClientOptions {
    let config = get_config();

    ClientOptions {
        username: config.username.clone(),
        password: config.password.clone(),
        proxy: config.proxy.clone(),
        retry_times: 3,
    }
}

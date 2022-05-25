// SPDX-License-Identifier: GPL-3.0-only
use crate::model::Config;
use anyhow::Result;
use std::fs;
use tracing::{error, info};

static CONFIG_FILE_PATH: &str = "config.json";

pub fn init_config() -> Result<Config> {
    info!("Loading config file");
    serde_json::from_str::<Config>(&fs::read_to_string(CONFIG_FILE_PATH)?).map_err(|e| {
        error!("Failed to load config file: {}", e);
        anyhow::anyhow!("init config err: {}", e)
    })
}

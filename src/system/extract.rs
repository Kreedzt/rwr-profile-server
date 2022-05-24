use crate::constant::{QUICK_ITEMS_JSON_FILE_NAME, RANKS_JSON_FILE_NAME};

use super::model::{QuickItem, RankItem};
use anyhow::{anyhow, Result};
use serde_json;
use std::fs;

pub fn get_quick_items_data(data_path: &str) -> Result<Vec<QuickItem>> {
    let file_name = format!("{}/{}", data_path, QUICK_ITEMS_JSON_FILE_NAME);
    let file = fs::read_to_string(&file_name)?;

    let quick_items_json: Vec<QuickItem> = serde_json::from_str(&file)?;

    Ok(quick_items_json)
}


pub fn get_ranks_data(data_path: &str) -> Result<Vec<RankItem>> {
    let file_name = format!("{}/{}", data_path, RANKS_JSON_FILE_NAME);
    let file = fs::read_to_string(&file_name)?;

    let ranks_json: Vec<RankItem> = serde_json::from_str(&file)?;

    Ok(ranks_json)
}

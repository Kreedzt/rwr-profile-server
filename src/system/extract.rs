use super::model::QuickItem;
use anyhow::{anyhow, Result};
use serde_json;
use std::fs;

pub fn get_quick_items_data(data_path: &str) -> Result<Vec<QuickItem>> {
    let file_name = format!("{}/quick_items.json", data_path);
    let file = fs::read_to_string(&file_name)?;

    let quick_items_json: Vec<QuickItem> = serde_json::from_str(&file)?;

    Ok(quick_items_json)
}

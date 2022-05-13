use super::model::QuickItem;
use anyhow::{anyhow, Result};
use serde_json;
use std::{fs, io::Write};

pub fn save_quick_items_to_file(
    data_path: &str,
    quick_items: &Vec<QuickItem>
) -> Result<()> {
    let file_name = format!("{}/quick_items.json", data_path);

    let json_str = serde_json::to_string(quick_items)?;

    let mut file = fs::File::create(&file_name)?;

    file.write_all(json_str.as_bytes())?;

    Ok(())
}

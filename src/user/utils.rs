// SPDX-License-Identifier: GPL-3.0-only
use super::model::{User, Users};
use crate::constant::USERS_JSON_FILE_NAME;
use crate::user::extract::get_user_profile_id;
use crate::AppData;
use anyhow::{anyhow, Result};
use serde_json;
use std::fs;
use std::io::Write;
use tracing_log::log::info;

pub fn get_user_json_data(data_path: &str) -> Result<Users> {
    let file_name = format!("{}/{}", data_path, USERS_JSON_FILE_NAME);
    let file = fs::read_to_string(&file_name)?;

    let user_json: Users = serde_json::from_str(&file)?;

    Ok(user_json)
}

pub fn validate_user(username: &str, password: &str, data_path: &str) -> Result<()> {
    let user_json_data = get_user_json_data(data_path)?;

    let res = user_json_data.user_list.iter().find(|x| {
        if x.name == username && password == x.password {
            return true;
        }
        return false;
    });

    return if let Some(_) = res {
        Ok(())
    } else {
        Err(anyhow!("not correct"))
    };
}

pub fn update_user_list(user_list: Vec<User>, data_path: &str) -> Result<()> {
    let file_name = format!("{}/{}", data_path, USERS_JSON_FILE_NAME);

    let mut user_json: Users = get_user_json_data(&data_path)?;

    user_json.user_list = user_list;

    let mut file = fs::File::create(&file_name)?;

    let json_str = serde_json::to_string(&user_json)?;

    file.write_all(json_str.as_bytes())?;

    Ok(())
}

pub fn get_user_info(username: &str, data_path: &str) -> Result<User> {
    let user_json: Users = get_user_json_data(data_path)?;

    let res = user_json.user_list.iter().find(|x| x.name == username);

    if let Some(user) = res {
        return Ok(User {
            name: user.name.clone(),
            password: user.password.clone(),
            admin: user.admin,
            user_id: user.user_id,
        });
    } else {
        return Err(anyhow!("User not found"));
    }
}

pub fn check_user_in_user_list(username: &str, data_path: &str) -> Result<bool> {
    let file_name = format!("{}/{}", data_path, USERS_JSON_FILE_NAME);

    info!("check_user_in_user_list:: file_name: {}", file_name);

    let file = std::fs::read_to_string(file_name)?;

    let user_json: Users = serde_json::from_str(&file)?;

    let res = user_json.user_list.iter().find(|x| x.name == username);

    if let Some(user) = res {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

pub fn get_user_id_in_profiles(username: &str, profile_path: &str) -> Result<bool> {
    let path = format!("{}/{}.profile", profile_path, username);

    Ok(true)
}

pub fn register_user(username: &str, password: &str, config: &AppData) -> Result<u64> {
    let is_in_user_list = check_user_in_user_list(username, &config.server_data_folder_path)?;

    if is_in_user_list {
        return Err(anyhow!("user already exists"));
    }

    let profile_id = get_user_profile_id(username, &config.rwr_profile_folder_path);

    return match profile_id {
        Ok(id) => Ok(id),
        Err(e) => Err(e),
    };
}

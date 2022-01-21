use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub rwr_profile_folder_path: String,
    pub server_data_folder_path: String,
    pub server_log_folder_path: String,
}

#[derive(Debug)]
pub struct AppData {
    pub rwr_profile_folder_path: String,
    pub server_data_folder_path: String,
    pub server_log_folder_path: String,
    pub user_json_lock: std::sync::Mutex<u8>,
}
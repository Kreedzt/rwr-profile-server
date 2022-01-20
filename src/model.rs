use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub rwr_profile_folder_path: String,
    pub server_data_folder_path: String,
    pub server_log_folder_path: String,
}
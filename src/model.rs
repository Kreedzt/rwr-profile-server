use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseJson {
    pub status: i32,
    pub code: i32,
    pub message: String,
}


impl Default for ResponseJson {
    fn default() -> Self {
        Self {
            status: 200,
            code: 0,
            message: String::from("ok")
        }
    }
}

impl ResponseJson {
    pub fn new(msg: &str) -> Self {
        let mut new_self = Self::default();

        new_self.message = String::from(msg);

        new_self
    }

    pub fn set_err_msg(&self, msg: &str) -> Self {
        Self {
            status: 400,
            code: -1,
            message: String::from(msg)
        }
    }

    pub fn set_successful_msg(&self, msg: &str) -> Self {
        Self {
            status: 200,
            code: 0,
            message: String::from(msg)
        }
    }
}
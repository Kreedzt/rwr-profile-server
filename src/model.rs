// SPDX-License-Identifier: GPL-3.0-only
use super::person::model::Person;
use super::profile::model::Profile;
use super::system::model::RankItem;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub rwr_profile_folder_path: String,
    pub server_data_folder_path: String,
    pub server_log_folder_path: String,
    pub server_upload_temp_folder_path: String,
    pub server_hourly_request: bool,
    pub port: u32,
}

#[derive(Debug)]
pub struct AppData {
    pub rwr_profile_folder_path: String,
    pub server_data_folder_path: String,
    pub server_log_folder_path: String,
    pub server_upload_temp_folder_path: String,
    pub user_json_lock: Mutex<u8>,
    // query_all snapshot
    pub snapshot_data: Mutex<Vec<(u64, Person, Profile)>>,
    // cache string
    pub snapshot_str: Mutex<String>,
    // cache time
    pub snapshot_time: Mutex<String>,
    // cache ranks
    pub snapshot_ranks: Mutex<Vec<RankItem>>
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
            message: String::from("ok"),
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
            message: String::from(msg),
        }
    }

    pub fn set_successful_msg(&self, msg: &str) -> Self {
        Self {
            status: 200,
            code: 0,
            message: String::from(msg),
        }
    }
}

// impl actix_web::error::ResponseError for ResponseJson {
//     fn error_response(&self) -> actix_web::HttpResponse {
//         HttpResponseBuilder::new(self.status_code())
//             .json(self)
//     }

//     fn status_code(&self) -> actix_web::http::StatusCode {
//         actix_web::http::StatusCode::BAD_REQUEST
//     }
// }

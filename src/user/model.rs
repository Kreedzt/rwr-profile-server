use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub user_id: u64,
    pub password: String,
    pub admin: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub user_list: Vec<User>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::new(),
            password: String::new(),
            user_id: 0,
            admin: 0,
        }
    }
}

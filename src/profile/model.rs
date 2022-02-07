use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub game_version: String,
    pub username: String,
    pub sid: String,
    pub rid: String,
    pub squad_tag: String,
    pub color: String,
    // TODO: 暂时只解析到此
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            game_version: String::new(),
            username: String::new(),
            sid: String::new(),
            rid: String::new(),
            squad_tag: String::new(),
            color: String::new(),
        }
    }
}

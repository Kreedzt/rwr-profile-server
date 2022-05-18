use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickItem {
    pub class: i8,
    pub index: i32,
    pub key: String,
    pub label: String,
}

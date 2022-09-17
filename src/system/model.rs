// SPDX-License-Identifier: GPL-3.0-only
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuickItem {
    pub class: i8,
    pub index: i32,
    pub key: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RankItem {
    pub xp: f64,
    pub name: String,
}

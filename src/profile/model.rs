use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stats {
    pub kills: u64,
    pub deaths: u64,
    pub time_played: f64,
    pub player_kills: u64,
    pub team_kills: u64,
    pub longest_kill_streak: u64,
    pub targets_destroyed: u64,
    pub vehicles_destroyed: u64,
    pub soldiers_healed: u64,
    pub times_got_healed: u64,
    pub distance_moved: f64,
    pub shots_fired: u64,
    pub throwables_thrown: u64,
    pub rank_progression: f64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub game_version: String,
    pub username: String,
    pub sid: String,
    pub rid: String,
    pub squad_tag: String,
    pub color: String,
    pub stats: Stats
    // TODO: 暂时只解析到此
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            kills: 0,
            deaths: 0,
            time_played: 0.0,
            player_kills: 0,
            team_kills: 0,
            longest_kill_streak: 0,
            targets_destroyed: 0,
            vehicles_destroyed: 0,
            soldiers_healed: 0,
            times_got_healed: 0,
            distance_moved: 0.0,
            shots_fired: 0,
            throwables_thrown: 0,
            rank_progression: 0.0
        }
    }
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
            stats: Stats::default()
        }
    }
}

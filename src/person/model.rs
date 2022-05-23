use serde::{Deserialize, Serialize};

use crate::{
    constant::{MAX_DEFAULT_BACKPACK_LEN, MAX_DEFAULT_STASH_LEN},
    profile::model::Profile,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StashItemTag {
    pub class: i8,
    pub index: i32,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemTag {
    pub slot: i8,
    pub index: i32,
    pub amount: i8,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTag {
    pub moving: i32,
    pub target: String,
    pub class: i8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub max_authority_reached: f32,
    pub authority: f32,
    pub job_points: f32,
    // TODO unkown
    pub faction: String,
    pub name: String,
    pub version: String,
    pub alive: i8,
    pub soldier_group_id: i8,
    pub soldier_group_name: String,
    pub block: String,
    pub squad_size_setting: f32,

    pub order: OrderTag,
    pub item_list: Vec<ItemTag>,
    // 1.92 新增: 背包容量
    pub backpack_hard_capacity: u16,
    // 1.92 新增: 仓库容量
    pub stash_hard_capacity: u16,
    pub backpack_item_list: Vec<StashItemTag>,
    pub stash_item_list: Vec<StashItemTag>,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            max_authority_reached: 0.0,
            authority: 0.0,
            job_points: 0.0,
            faction: String::new(),
            name: String::new(),
            version: String::new(),
            alive: 0,
            soldier_group_id: 0,
            soldier_group_name: String::from("default"),
            block: String::new(),
            squad_size_setting: 0.0,
            order: OrderTag::default(),
            item_list: vec![],
            backpack_hard_capacity: MAX_DEFAULT_BACKPACK_LEN,
            stash_hard_capacity: MAX_DEFAULT_STASH_LEN,
            backpack_item_list: vec![],
            stash_item_list: vec![],
        }
    }
}

impl From<&Person> for Person {
    fn from(p: &Person) -> Self {
        Person {
            max_authority_reached: p.max_authority_reached,
            authority: p.authority,
            job_points: p.job_points,
            faction: p.faction.to_owned(),
            name: p.name.to_owned(),
            version: p.version.to_owned(),
            alive: p.alive,
            soldier_group_id: p.soldier_group_id,
            soldier_group_name: p.soldier_group_name.to_owned(),
            block: p.block.to_owned(),
            squad_size_setting: p.squad_size_setting,
            order: OrderTag {
                moving: p.order.moving,
                class: p.order.class,
                target: p.order.target.to_owned(),
            },
            item_list: p.item_list.to_vec(),
            backpack_hard_capacity: p.backpack_hard_capacity,
            stash_hard_capacity: p.stash_hard_capacity,
            backpack_item_list: p.backpack_item_list.to_vec(),
            stash_item_list: p.stash_item_list.to_vec(),
        }
    }
}

impl Default for OrderTag {
    fn default() -> Self {
        Self {
            moving: 0,
            target: String::new(),
            class: 0,
        }
    }
}

impl Default for ItemTag {
    fn default() -> Self {
        Self {
            slot: 0,
            index: 0,
            amount: 0,
            key: String::new(),
        }
    }
}

impl Default for StashItemTag {
    fn default() -> Self {
        Self {
            class: 0,
            index: 0,
            key: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePersonReq {
    pub person: Person,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetXpReq {
    pub authority: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupInfo {
    pub group_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InsertSelectedPersonBackpackReq {
    pub profile_id_list: Vec<u64>,
    pub backpack_item_list: Vec<StashItemTag>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAllPersonSoldierGroupReq {
    pub group: String,
    pub cost: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSelectedPersonSoldierGroupReq {
    pub profile_id_list: Vec<u64>,
    pub group: String,
    pub cost: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePersonSoldierGroupRes {
    pub error_profile_list: Vec<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteSelectedPersonItemListReq {
    pub profile_id_list: Vec<u64>,
    pub item_list: Vec<String>,
}

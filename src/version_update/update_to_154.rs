use crate::{
    constant::{MAX_DEFAULT_BACKPACK_LEN, MAX_DEFAULT_STASH_LEN},
    person::model::{ItemGroupTag, ItemTag, OrderTag, Person},
    person::save::save_person_to_file,
};
use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, fs, io, str};
use regex::Regex;

struct StashItemTag {
    pub class: i8,
    pub index: i32,
    pub key: String,
}

impl Default for StashItemTag {
    fn default() -> Self {
        Self {
            class: 0,
            index: -1,
            key: "".to_string(),
        }
    }
}

struct Person_V153 {
    pub max_authority_reached: f32,
    pub authority: f32,
    pub job_points: f32,
    // TODO unkown
    pub faction: String,
    pub name: String,
    pub version: u16,
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

impl Default for Person_V153 {
    fn default() -> Self {
        Self {
            max_authority_reached: 0.0,
            authority: 0.0,
            job_points: 0.0,
            faction: String::new(),
            name: String::new(),
            version: 153,
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

pub async fn update_person(folder_path: &str, id: u64) -> Result<()> {
    let mut person = Person_V153::default();

    let path = format!("{}/{}.person", folder_path, id);

    let mut reader = Reader::from_file(path)?;

    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut is_in_person = false;
    let mut is_in_stash = false;
    let mut is_in_backpack = false;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                // println!("start e.name: {:?}", str::from_utf8(e.name()).unwrap());
                match e.name() {
                    b"person" => {
                        is_in_person = true;

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            match attr_key {
                                b"max_authority_reached" => {
                                    person.max_authority_reached = attr_value.parse()?;
                                }
                                b"authority" => {
                                    person.authority = attr_value.parse()?;
                                }
                                b"job_points" => {
                                    person.job_points = attr_value.parse()?;
                                }
                                b"faction" => {
                                    person.faction = attr_value;
                                }
                                b"name" => {
                                    person.name = attr_value;
                                }
                                b"version" => {
                                    person.version = attr_value.parse()?;
                                }
                                b"alive" => {
                                    person.alive = attr_value.parse()?;
                                }
                                b"soldier_group_id" => {
                                    person.soldier_group_id = attr_value.parse()?;
                                }
                                b"soldier_group_name" => {
                                    person.soldier_group_name = attr_value;
                                }
                                b"block" => {
                                    person.block = attr_value;
                                }
                                b"squad_size_setting" => {
                                    person.squad_size_setting = attr_value.parse()?;
                                }
                                b"order" => {
                                    let mut order_item = OrderTag::default();
                                }
                                _ => (),
                            }
                        }
                        // println!("This is person");
                    }
                    b"stash" => {
                        is_in_stash = true;

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            match attr_key {
                                b"hard_capacity" => {
                                    person.stash_hard_capacity = attr_value.parse()?;
                                }
                                _ => {}
                            }
                        }
                    }
                    b"backpack" => {
                        is_in_backpack = true;

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            match attr_key {
                                b"hard_capacity" => {
                                    person.backpack_hard_capacity = attr_value.parse()?;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                // println!("empty e.name: {:?}", str::from_utf8(e.name())?);

                match e.name() {
                    b"order" => {
                        let mut order_item = OrderTag::default();

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            // println!("attr: {}, value: {}", str::from_utf8(attr_key)?, attr_value);

                            match attr_key {
                                b"moving" => {
                                    order_item.moving = attr_value.parse()?;
                                }
                                b"target" => {
                                    order_item.target = attr_value;
                                }
                                b"class" => {
                                    order_item.class = attr_value.parse()?;
                                }
                                _ => (),
                            }
                        }

                        person.order = order_item;
                    }
                    b"item" => {
                        if is_in_backpack || is_in_stash {
                            let mut item_tag = StashItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr?;
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader)?;
                                let attr_key = attr_unwrap_res.key;

                                // println!(
                                //     "attr: {}, value: {}",
                                //     str::from_utf8(attr_key)?,
                                //     attr_value
                                // );

                                match attr_key {
                                    b"class" => {
                                        item_tag.class = attr_value.parse()?;
                                    }
                                    b"index" => {
                                        item_tag.index = attr_value.parse()?;
                                    }
                                    b"key" => {
                                        item_tag.key = attr_value;
                                    }
                                    _ => (),
                                }
                            }

                            if is_in_backpack {
                                person.backpack_item_list.push(item_tag);
                            } else {
                                person.stash_item_list.push(item_tag);
                            }
                        } else {
                            let mut item_tag = ItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr?;
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader)?;
                                let attr_key = attr_unwrap_res.key;

                                // println!(
                                //     "attr: {}, value: {}",
                                //     str::from_utf8(attr_key)?,
                                //     attr_value
                                // );

                                match attr_key {
                                    b"slot" => {
                                        item_tag.slot = attr_value.parse()?;
                                    }
                                    b"index" => {
                                        item_tag.index = attr_value.parse()?;
                                    }
                                    b"amount" => {
                                        item_tag.amount = attr_value.parse()?;
                                    }
                                    b"key" => {
                                        item_tag.key = attr_value;
                                    }
                                    _ => (),
                                }
                            }

                            person.item_list.push(item_tag);
                        }
                    }
                    b"backpack" => {}
                    _ => (),
                }
            }
            Ok(Event::End(e)) => {
                // DEBUG
                // println!("end e.name: {:?}", str::from_utf8(e.name())?);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {} : {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    let mut new_person: Person = Person::default();

    new_person.max_authority_reached = person.max_authority_reached;
    new_person.authority = person.authority;
    new_person.job_points = person.job_points;
    new_person.faction = person.faction;
    new_person.name = person.name;
    new_person.alive = person.alive;
    new_person.soldier_group_id = person.soldier_group_id;
    new_person.soldier_group_name = person.soldier_group_name;
    new_person.order = person.order;
    new_person.item_list = person.item_list;
    new_person.stash_hard_capacity = person.stash_hard_capacity;
    new_person.backpack_hard_capacity = person.backpack_hard_capacity;

    let mut backpack_item_map: HashMap<String, ItemGroupTag> =
        HashMap::with_capacity(person.backpack_item_list.len());
    for backpack_item in person.backpack_item_list {
        backpack_item_map
            .entry(backpack_item.key.clone())
            .and_modify(|item| item.amount += 1)
            .or_insert(ItemGroupTag {
                index: backpack_item.index,
                class: backpack_item.class,
                amount: 1,
                key: backpack_item.key,
            });
    }

    let item_group_list: Vec<ItemGroupTag> = backpack_item_map
        .values()
        .map(|item| item.clone())
        .collect::<Vec<ItemGroupTag>>();
    new_person.backpack_item_list = item_group_list;

    let mut stash_item_map: HashMap<String, ItemGroupTag> =
        HashMap::with_capacity(person.stash_item_list.len());
    for stash_item in person.stash_item_list {
        stash_item_map
            .entry(stash_item.key.clone())
            .and_modify(|item| item.amount += 1)
            .or_insert(ItemGroupTag {
                index: stash_item.index,
                class: stash_item.class,
                amount: 1,
                key: stash_item.key,
            });
    }

    let item_group_list: Vec<ItemGroupTag> = stash_item_map
        .values()
        .map(|item| item.clone())
        .collect::<Vec<ItemGroupTag>>();
    new_person.stash_item_list = item_group_list;

    save_person_to_file(folder_path, id, &new_person)?;

    Ok(())
}

pub async fn update_profile(folder_path: &str, id: u64) -> Result<()> {
    // replace game_version text only

    let path = format!("{}/{}.profile", folder_path, id);

    let file_content = tokio::fs::read_to_string(path.clone()).await?;

    let search_str = r#"game_version="\d+""#;

    let re = Regex::new(search_str).unwrap();

    let new_text = "game_version=\"154\"";

    let result = re.replace(&file_content, new_text).to_string();

    tokio::fs::write(path, result).await?;

    Ok(())
}

// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    constant::{MAX_DEFAULT_STASH_LEN, MAX_PERSON_FILE_VERSION},
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use anyhow::{anyhow, Result};
use quick_xml::{events::Event, Reader};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, fs, io, str};
use tracing::info;

use super::model::{ItemGroupTag};

pub fn extract_person(id: u64, folder_path: &str) -> Result<Person> {
    // 优先以最高版本解析
    let mut person = Person::default();

    // 当前解析的版本
    let mut this_version = MAX_PERSON_FILE_VERSION;

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
                    // item_group 仅在 1.94 中存在, 且仅在 stash/backpack 中
                    b"item_group" => {
                        let mut item_group = ItemGroupTag::default();

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            // println!(
                            //     "attr: {}, value: {}",
                            //     str::from_utf8(attr_key)?,
                            //     attr_value
                            // );

                            match attr_key {
                                b"class" => {
                                    item_group.class = attr_value.parse()?;
                                }
                                b"index" => {
                                    item_group.index = attr_value.parse()?;
                                }
                                b"key" => {
                                    item_group.key = attr_value.parse()?;
                                }
                                b"amount" => {
                                    item_group.amount = attr_value.parse()?;
                                }
                                _ => (),
                            }
                        }

                        if is_in_backpack {
                            person.backpack_item_list.push(item_group);
                        } else if is_in_stash {
                            person.stash_item_list.push(item_group);
                        }
                    }
                    b"item" => {
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

    if person.version != MAX_PERSON_FILE_VERSION {
        anyhow!(
            "{} Person file \"version\" not correct, expected: {}, got: {}",
            id,
            MAX_PERSON_FILE_VERSION,
            person.version
        );
    }

    Ok(person)
}

pub fn extract_all_person(folder_path: &str) -> Result<Vec<(u64, Person)>> {
    let entries = fs::read_dir(folder_path)?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".profile")
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    let v: Result<Vec<(u64, Person)>> = entries
        .into_par_iter()
        .map(|path| {
            let file_name = path
                .file_name()
                .ok_or(anyhow!("path read file_name error: {:?}", path))?
                .to_str()
                .ok_or(anyhow!("path file_name to str error: {:?}", path))?;

            let last_list = file_name.split(".").collect::<Vec<_>>();

            let id: u64 = last_list
                .first()
                .ok_or(anyhow!("last list get first error: {:?}", path))?
                .parse()?;

            let person = extract_person(id, folder_path)?;

            Ok((id, person))
        })
        .collect();

    Ok(v?)
}

pub fn extract_all_person_and_profiles(folder_path: &str) -> Result<Vec<(u64, Person, Profile)>> {
    let entries = fs::read_dir(folder_path)?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".profile")
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    let v: Result<Vec<(u64, Person, Profile)>> = entries
        .into_par_iter()
        .map(|path| {
            let file_name = path
                .file_name()
                .ok_or(anyhow!("path read file_name error: {:?}", path))?
                .to_str()
                .ok_or(anyhow!("path file_name to str error: {:?}", path))?;

            let last_list = file_name.split(".").collect::<Vec<_>>();

            let id: u64 = last_list
                .first()
                .ok_or(anyhow!("last list get first error: {:?}", path))?
                .parse()?;

            let person = extract_person(id, folder_path)?;

            let profile = extract_profile(id, folder_path)?;

            Ok((id, person, profile))
        })
        .collect();

    Ok(v?)
}

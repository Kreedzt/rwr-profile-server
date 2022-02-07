use crate::{
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use anyhow::Result;
use quick_xml::{events::Event, Reader};
use std::{fs, io, str};
use tracing::info;

pub fn extract_person(id: u64, folder_path: &str) -> Result<Person> {
    let mut person = Person::default();

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
                                    person.version = attr_value;
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
                        println!("This is person");
                    }
                    b"stash" => {
                        is_in_stash = true;
                        println!("This is stash");
                    }
                    b"backpack" => {
                        is_in_backpack = true;
                        println!("This is backpack");
                    }
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                println!("empty e.name: {:?}", str::from_utf8(e.name())?);

                match e.name() {
                    b"order" => {
                        let mut order_item = OrderTag::default();

                        for attr in e.attributes() {
                            let attr_unwrap_res = attr?;
                            let attr_value = attr_unwrap_res.unescape_and_decode_value(&reader)?;
                            let attr_key = attr_unwrap_res.key;

                            println!("attr: {}, value: {}", str::from_utf8(attr_key)?, attr_value);

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
                        if is_in_backpack {
                            let mut backpack_item = StashItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr?;
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader)?;
                                let attr_key = attr_unwrap_res.key;

                                println!(
                                    "attr: {}, value: {}",
                                    str::from_utf8(attr_key)?,
                                    attr_value
                                );

                                match attr_key {
                                    b"class" => {
                                        backpack_item.class = attr_value.parse()?;
                                    }
                                    b"index" => {
                                        backpack_item.index = attr_value.parse()?;
                                    }
                                    b"key" => {
                                        backpack_item.key = attr_value.parse()?;
                                    }
                                    _ => (),
                                }
                            }

                            person.backpack_item_list.push(backpack_item);
                        } else if is_in_stash {
                            let mut stash_item = StashItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr?;
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader)?;
                                let attr_key = attr_unwrap_res.key;

                                println!(
                                    "attr: {}, value: {}",
                                    str::from_utf8(attr_key)?,
                                    attr_value
                                );

                                match attr_key {
                                    b"class" => {
                                        stash_item.class = attr_value.parse()?;
                                    }
                                    b"index" => {
                                        stash_item.index = attr_value.parse()?;
                                    }
                                    b"key" => {
                                        stash_item.key = attr_value.parse()?;
                                    }
                                    _ => (),
                                }
                            }

                            person.stash_item_list.push(stash_item);
                        } else {
                            let mut item_tag = ItemTag::default();

                            for attr in e.attributes() {
                                let attr_unwrap_res = attr?;
                                let attr_value =
                                    attr_unwrap_res.unescape_and_decode_value(&reader)?;
                                let attr_key = attr_unwrap_res.key;

                                println!(
                                    "attr: {}, value: {}",
                                    str::from_utf8(attr_key)?,
                                    attr_value
                                );

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
                println!("end e.name: {:?}", str::from_utf8(e.name())?);
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {} : {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    Ok(person)
}

pub fn extract_all_person(folder_path: &str) -> Result<Vec<(u64, Person)>> {
    let mut v: Vec<(u64, Person)> = Vec::new();

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

    for path in entries.into_iter() {
        let reader_path = path.into_os_string().into_string().unwrap();

        let path_string = reader_path.clone();
        let path_list = path_string.split("\\").collect::<Vec<_>>();

        let last_path = path_list.last().unwrap();
        let last_list = last_path.split(".").collect::<Vec<_>>();
        let id: u64 = last_list.first().unwrap().parse()?;

        info!("extract item: {} / {}", id, last_path);
        let person = extract_person(id, folder_path)?;

        v.push((id, person));
    }

    Ok(v)
}

pub fn extract_all_person_and_profiles(folder_path: &str) -> Result<Vec<(u64, Person, Profile)>> {
    let mut v: Vec<(u64, Person, Profile)> = Vec::new();

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

    for path in entries.into_iter() {
        let reader_path = path.into_os_string().into_string().unwrap();

        let path_string = reader_path.clone();
        let path_list = path_string.split("\\").collect::<Vec<_>>();

        let last_path = path_list.last().unwrap();
        let last_list = last_path.split(".").collect::<Vec<_>>();
        let id: u64 = last_list.first().unwrap().parse()?;

        info!("extract item: {} / {}", id, last_path);
        let person = extract_person(id, folder_path)?;

        let profile = extract_profile(id, folder_path)?;

        v.push((id, person, profile));
    }

    Ok(v)
}

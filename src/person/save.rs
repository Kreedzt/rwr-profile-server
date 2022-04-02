use super::model::Person;
use crate::constant::MAX_BACKPACK_LEN;
use crate::person::{extract::extract_person, model::StashItemTag};
use anyhow::Result;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::{Cursor, Write};
use tracing::{error, warn};

pub fn save_person(p: &Person) -> Result<String> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

    let mut person_tag = BytesStart::owned(b"person".to_owned(), "person".len());

    person_tag.push_attribute((
        "max_authority_reached",
        p.max_authority_reached.to_string().as_str(),
    ));
    person_tag.push_attribute(("authority", p.authority.to_string().as_str()));
    person_tag.push_attribute(("job_points", p.job_points.to_string().as_str()));
    person_tag.push_attribute(("faction", p.faction.to_string().as_str()));
    person_tag.push_attribute(("name", p.name.as_str()));
    person_tag.push_attribute(("version", p.version.as_str()));
    person_tag.push_attribute(("alive", p.alive.to_string().as_str()));
    person_tag.push_attribute(("soldier_group_id", p.soldier_group_id.to_string().as_str()));
    person_tag.push_attribute(("soldier_group_name", p.soldier_group_name.as_str()));
    person_tag.push_attribute(("block", p.block.as_str()));
    person_tag.push_attribute((
        "squad_size_setting",
        p.squad_size_setting.to_string().as_str(),
    ));

    writer.write_event(Event::Start(person_tag))?;

    let mut order_tag = BytesStart::owned(b"order".to_owned(), "order".len());

    order_tag.push_attribute(("moving", p.order.moving.to_string().as_str()));
    order_tag.push_attribute(("target", p.order.target.to_string().as_str()));
    order_tag.push_attribute(("class", p.order.class.to_string().as_str()));

    writer.write_event(Event::Empty(order_tag))?;

    for item in p.item_list.iter() {
        let mut item_tag = BytesStart::owned(b"item".to_owned(), "item".len());

        item_tag.push_attribute(("slot", item.slot.to_string().as_str()));
        item_tag.push_attribute(("index", item.index.to_string().as_str()));
        item_tag.push_attribute(("amount", item.amount.to_string().as_str()));
        item_tag.push_attribute(("key", item.key.as_str()));

        writer.write_event(Event::Empty(item_tag))?;
    }

    let stash_tag = BytesStart::owned(b"stash".to_owned(), "stash".len());

    writer.write_event(Event::Start(stash_tag))?;

    for item in p.stash_item_list.iter() {
        let mut stash_item_tag = BytesStart::owned(b"item".to_owned(), "item".len());

        stash_item_tag.push_attribute(("class", item.class.to_string().as_str()));
        stash_item_tag.push_attribute(("index", item.index.to_string().as_str()));
        stash_item_tag.push_attribute(("key", item.key.as_str()));

        writer.write_event(Event::Empty(stash_item_tag))?;
    }

    writer.write_event(Event::End(BytesEnd::borrowed(b"stash")))?;

    let backpack_tag = BytesStart::owned(b"backpack".to_owned(), "backpack".len());

    if p.backpack_item_list.len() == 0 {
        writer.write_event(Event::Empty(backpack_tag))?;
    } else {
        writer.write_event(Event::Start(backpack_tag))?;

        for item in p.backpack_item_list.iter() {
            let mut backpack_item_tag = BytesStart::owned(b"item".to_owned(), "item".len());

            backpack_item_tag.push_attribute(("class", item.class.to_string().as_str()));
            backpack_item_tag.push_attribute(("index", item.index.to_string().as_str()));
            backpack_item_tag.push_attribute(("key", item.key.as_str()));

            writer.write_event(Event::Empty(backpack_item_tag))?;
        }

        writer.write_event(Event::End(BytesEnd::borrowed(b"backpack")))?;
    }

    writer.write_event(Event::End(BytesEnd::borrowed(b"person")))?;

    let result = String::from_utf8(writer.into_inner().into_inner())?;

    Ok(result)
}

pub fn save_person_to_file(path: &str, id: u64, person: &Person) -> Result<()> {
    let res_person_str = save_person(person)?;

    let target_path = format!("{}/{}.person", path, id);

    let mut file = std::fs::File::create(target_path)?;

    file.write_all(res_person_str.as_bytes())?;

    Ok(())
}

pub async fn insert_person_list_backpack_to_file(
    path: &str,
    all_person_list: &Vec<(u64, Person)>,
    item_list: &Vec<StashItemTag>,
) -> Result<()> {
    let new_all_person_list: Vec<(u64, Person)> = all_person_list
        .into_iter()
        .map(|info| {
            let (_id, _person) = info;
            let id: u64 = _id.clone();
            let mut new_person: Person = _person.clone();

            // 若超出, 终止操作
            if new_person.backpack_item_list.len() + item_list.len() > MAX_BACKPACK_LEN.into() {
                error!("person id: {} backpack over 255", id);
                return (id, new_person);
            }

            new_person.backpack_item_list.extend(item_list.to_owned());

            (id, new_person)
        })
        .collect();

    // for data in new_all_person_list.into_iter() {
    //     save_person_to_file(path, data.0, &data.1)?;
    // }

    let folder_path = path.to_string();


    let future_vec = new_all_person_list.into_iter().map(|info| {
        let cloned_folder_path = folder_path.clone();
        return tokio::spawn(
            async move { save_person_to_file(&cloned_folder_path, info.0, &info.1) },
        );
    });

    futures::future::try_join_all(future_vec).await?;

    Ok(())
}

pub async fn update_person_list_soldider_group_to_file(
    path: &str,
    all_person_list: &Vec<(u64, Person)>,
    group: &str,
    cost: f32,
) -> Result<Vec<u64>> {
    let mut err_profile_id_vec = vec![];

    let new_all_person_list: Vec<(u64, Person)> = all_person_list
        .into_iter()
        .map(|info| {
            let (_id, _person) = info;
            let id: u64 = _id.clone();
            let mut new_person: Person = _person.clone();

            // 若 RP 不足, 终止操作
            if new_person.job_points < cost {
                error!("person id: {} rp < cost: {}", id, cost);
                err_profile_id_vec.push(id);
                return (id, new_person);
            }

            new_person.job_points = new_person.job_points - cost;
            new_person.soldier_group_name = group.to_string();

            (id, new_person)
        })
        .collect();

    let folder_path = path.to_string();

    let future_vec = new_all_person_list.into_iter().map(|info| {
        let cloned_folder_path = folder_path.clone();
        return tokio::spawn(
            async move { save_person_to_file(&cloned_folder_path, info.0, &info.1) },
        );
    });

    futures::future::try_join_all(future_vec).await?;

    Ok(err_profile_id_vec)
}

use super::extract::extract_person;
use crate::{
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use anyhow::Result;
use futures::{self, FutureExt};
use tokio;
use tracing::info;

type ExtractAllType = (u64, Person, Profile);
type ExtractAllVec = Vec<ExtractAllType>;

pub async fn async_extract_all_person_and_profiles(folder_path: String) -> Result<ExtractAllVec> {
    let mut folder_entries = tokio::fs::read_dir(&folder_path).await?;

    let mut entries: Vec<u64> = Vec::new();

    while let Some(entry) = folder_entries.next_entry().await? {
        let file_name = entry.file_name();
        let file_name_str = String::from(file_name.to_str().unwrap());

        if file_name_str.ends_with(".profile") {
            let vec = file_name_str.split(".").collect::<Vec<_>>();

            let parse_res = vec.first().unwrap().parse();

            if let Ok(id) = parse_res {
                entries.push(id);
            }
        }
    }

    let mut v: ExtractAllVec = Vec::new();

    let person_future_vec = entries
        .clone()
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let person = extract_person(id, &cloned_folder_path).unwrap();
                person
            });
        })
        .collect::<Vec<_>>();

    let profile_future_vec = entries
        .clone()
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let profile = extract_profile(id, &cloned_folder_path).unwrap();
                profile
            });
        })
        .collect::<Vec<_>>();

    let person_vec = futures::future::try_join_all(person_future_vec).await?;
    let profile_vec = futures::future::try_join_all(profile_future_vec).await?;

    let mut res_v: ExtractAllVec = Vec::with_capacity(entries.len());

    for (index, id) in entries.into_iter().enumerate() {
        let person = person_vec.get(index).unwrap();
        let profile = profile_vec.get(index).unwrap();

        let push_item: ExtractAllType = (id, person.clone(), profile.clone());

        res_v.push(push_item);
    }

    Ok(res_v)
}

type ExtractPersonType = (u64, Person);
type ExtractPersonVec = Vec<ExtractPersonType>;

pub async fn async_extract_all_person(folder_path: String) -> Result<ExtractPersonVec> {
    let mut folder_entries = tokio::fs::read_dir(&folder_path).await?;

    let mut entries: Vec<u64> = Vec::new();

    while let Some(entry) = folder_entries.next_entry().await? {
        let file_name = entry.file_name();
        let file_name_str = String::from(file_name.to_str().unwrap());

        if file_name_str.ends_with(".person") {
            let vec = file_name_str.split(".").collect::<Vec<_>>();

            let id: u64 = vec.first().unwrap().parse().unwrap();

            entries.push(id);
        }
    }

    let person_future_vec = entries
        .clone()
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let person = extract_person(id, &cloned_folder_path).unwrap();
                (id, person)
            });
        })
        .collect::<Vec<_>>();

    let person_vec = futures::future::try_join_all(person_future_vec).await?;

    Ok(person_vec)
}

pub async fn async_extract_selected_person(
    folder_path: String,
    profile_id_list: Vec<u64>,
) -> Result<ExtractPersonVec> {
    let person_future_vec = profile_id_list
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let person = extract_person(id, &cloned_folder_path).unwrap();
                (id, person)
            });
        })
        .collect::<Vec<_>>();

    let person_vec = futures::future::try_join_all(person_future_vec).await?;

    Ok(person_vec)
}

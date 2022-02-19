use super::extract::extract_person;
use crate::{
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use actix_web;
use anyhow::Result;
use futures;
use quick_xml::{events::Event, Reader};
use std::sync::{Arc, Mutex};
use std::{borrow::Borrow, fs, io, str};
use tokio;
use tracing::info;

type ExtractAllType = (u64, Person, Profile);
type ExtractAllVec = Vec<ExtractAllType>;

pub async fn async_extract_all_person_and_profiles(folder_path: String) -> Result<ExtractAllVec> {
    let entries = fs::read_dir(&folder_path)?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".profile")
        })
        .map(|path| {
            let path = path.unwrap();
            let reader_path = path.to_str().unwrap();

            let path_string = String::from(reader_path);
            let path_list = path_string.split("\\").collect::<Vec<_>>();

            let last_path = path_list.last().unwrap();
            let last_list = last_path.split(".").collect::<Vec<_>>();
            let id: u64 = last_list.first().unwrap().parse().unwrap();
            id
        })
        .collect::<Vec<u64>>();

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

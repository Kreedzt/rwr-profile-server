// SPDX-License-Identifier: GPL-3.0-only
use std::borrow::Borrow;

use super::extract::extract_person;
use crate::{
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use anyhow::{anyhow, Result};
use futures::{self, FutureExt, SinkExt};
use rayon::prelude::*;
use tokio;
use tracing::{info, error};

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

    let person_future_vec = entries
        .clone()
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let person = extract_person(id, &cloned_folder_path);
                match person {
                    Ok(p) => {
                        Ok(p)
                    },
                    Err(e) => {
                        error!("error in extract_person call: {}, {:?}", id, e);
                        Err(e)
                    }
                }
            });
        })
        .collect::<Vec<_>>();

    let profile_future_vec = entries
        .clone()
        .into_iter()
        .map(|id| {
            let cloned_folder_path = folder_path.clone();

            return tokio::spawn(async move {
                let profile = extract_profile(id, &cloned_folder_path);
                match profile {
                    Ok(p) => {
                        Ok(p)
                    },
                    Err(e) => {
                        error!("error in extract_profile call: {}, {:?}", id, e);
                        Err(e)
                    }
                }
            });
        })
        .collect::<Vec<_>>();

    let person_vec = futures::future::try_join_all(person_future_vec).await?;
    let profile_vec = futures::future::try_join_all(profile_future_vec).await?;

    let res_v: Result<ExtractAllVec> = entries
        .into_par_iter()
        .enumerate()
        .map(|(index, id)| {
            // person
            let person = person_vec
                .get(index)
                .ok_or(anyhow!("error in profile_vec get: {}", index))?;

            let person = person
                .as_ref()
                .map_err(|err| anyhow!("ID: {} err in person extract: {:?}", id, err))
                .unwrap();

            let person = person.clone();

            // profile
            let profile = profile_vec
                .get(index)
                .ok_or(anyhow!("error in profile_vec get: {}", index))?;

            let profile = profile
                .as_ref()
                .map_err(|err| anyhow!("ID: {} err in profile extract: {:?}", id, err))
                .unwrap();

            let profile = profile.clone();

            let push_item: ExtractAllType = (id, person, profile);
            Ok(push_item)
        })
        .collect();

    Ok(res_v?)
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

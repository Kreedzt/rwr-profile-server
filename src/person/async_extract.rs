use crate::{
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use tokio;
use actix_web;
use anyhow::Result;
use quick_xml::{events::Event, Reader};
use super::extract::extract_person;
use std::{fs, io, str, borrow::Borrow};
use std::sync::{Arc, Mutex};
use tracing::info;
use futures;

pub async fn async_extract_all_person_and_profiles(folder_path: String) -> Result<Vec<(u64, Person, Profile)>> {
    let entries = fs::read_dir(&folder_path)?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| {
            path.as_ref()
                .unwrap()
                .display()
                .to_string()
                .ends_with(".profile")
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut v: Vec<(u64, Person, Profile)> = Vec::new();

    let shared_v = Arc::new(Mutex::new(v));

    let person_future_vec = entries.clone().into_iter().map(|path| {
        let reader_path = path.into_os_string().into_string().unwrap();

        let path_string = reader_path.clone();
        let path_list = path_string.split("\\").collect::<Vec<_>>();

        let last_path = path_list.last().unwrap();
        let last_list = last_path.split(".").collect::<Vec<_>>();
        let id: u64 = last_list.first().unwrap().parse().unwrap();

        info!("extract item: {} / {}", id, last_path);

        let cloned_folder_path = folder_path.clone();

        return tokio::spawn(async move {
            let person = extract_person(id, &cloned_folder_path).unwrap();
            person
        });

        // return actix_web::rt::spawn(async move {
        //     let person = extract_person(id, &cloned_folder_path).unwrap();
        //     person
        // });
    }).collect::<Vec<_>>();

    let profile_future_vec = entries.clone().into_iter().map(|path| {
        let reader_path = path.into_os_string().into_string().unwrap();

        let path_string = reader_path.clone();
        let path_list = path_string.split("\\").collect::<Vec<_>>();

        let last_path = path_list.last().unwrap();
        let last_list = last_path.split(".").collect::<Vec<_>>();
        let id: u64 = last_list.first().unwrap().parse().unwrap();

        // info!("extract item: {} / {}", id, last_path);

        let cloned_folder_path = folder_path.clone();


        return tokio::spawn(async move {
            let profile = extract_profile(id, &cloned_folder_path).unwrap();
            profile
        });

        // return actix_web::rt::spawn(async move {
        //     let profile = extract_profile(id, &cloned_folder_path).unwrap();
        //     profile
        // });
    }).collect::<Vec<_>>();

    let person_vec = futures::future::try_join_all(person_future_vec).await?;
    let profile_vec = futures::future::try_join_all(profile_future_vec).await?;

    let mut res_v = shared_v.lock().unwrap().clone();

    println!("{:?}", person_vec);

    // for (profile, index) in profile_vec.into_iter().enumerate() {
    //     let person = person_vec.get(index).unwrap();

    // }

    Ok(res_v)
}

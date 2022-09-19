use std::borrow::Borrow;

use super::update_to_154;
use crate::{
    person::extract::extract_person,
    person::model::{ItemTag, OrderTag, Person, StashItemTag},
    profile::{extract::extract_profile, model::Profile},
};
use anyhow::{anyhow, Result};
use futures::{self, FutureExt, SinkExt};
use rayon::prelude::*;
use tokio;
use tracing::{error, info};

async fn query_all_data(folder_path: String) -> Result<()> {
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
                        if (p.version < 154) {
                            // update from 153 to 154
                            update_to_154::update_person(&cloned_folder_path, id)
                                .await
                                .unwrap();
                        }
                        Ok(())
                    }
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
                        if p.game_version < 154 {
                            // update from 153 to 154
                            update_to_154::update_profile(&cloned_folder_path, id)
                                .await
                                .unwrap();
                        }
                        Ok(())
                    }
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

    Ok(())
}

pub async fn preupdate(folder_path: String) -> Result<()> {
    info!("Starting preupdate...");

    query_all_data(folder_path).await?;

    info!("Preupdate Completed.");

    Ok(())
}

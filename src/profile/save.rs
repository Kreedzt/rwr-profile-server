// SPDX-License-Identifier: GPL-3.0-only
use anyhow::Result;

use super::model::Profile;

pub async fn update_profile_list_squad_tag_to_file(
    path: &str,
    all_profile_list: &Vec<(u64, Profile)>,
    squad_tag: &str,
) -> Result<Vec<u64>> {
    let mut err_profile_id_vec = vec![];

    let new_all_profile_list: Vec<(u64, Profile)> = all_profile_list
        .into_iter()
        .map(|info| {
            let (_id, _profile) = info;
            let id: u64 = _id.clone();
            let mut new_profile: Profile = _profile.clone();

            new_profile.squad_tag = squad_tag.to_string();

            (id, new_profile)
        })
        .collect();

    let folder_path = path.to_string();

    let future_vec = new_all_profile_list.into_iter().map(|info| {
        let cloned_folder_path = folder_path.clone();

        return tokio::spawn(async move {
            // TODO
            // save_profile_to_file(&cloned_folder_path, info.0, &info.1)
        });
    });

    futures::future::try_join_all(future_vec).await?;

    Ok(err_profile_id_vec)
}

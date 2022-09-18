// SPDX-License-Identifier: GPL-3.0-only
use super::model::{
    InsertSelectedPersonBackpackReq, ItemGroupTag, Person, UpdateAllPersonSoldierGroupReq,
    UpdatePersonReq, UpdateSelectedPersonSoldierGroupReq,
};
use crate::model::ResponseJson;
use crate::person::async_extract::{
    async_extract_all_person, async_extract_all_person_and_profiles, async_extract_selected_person,
};
use crate::person::extract::extract_person;
use crate::person::model::{
    DeleteSelectedPersonItemListReq, GroupInfo, ResetXpReq, UpdatePersonSoldierGroupRes,
};
use crate::person::save::{
    delete_person_item_list_to_file, insert_person_list_backpack_to_file, save_person_to_file,
    update_person_list_soldider_group_to_file,
};
use crate::AppData;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use futures_util::TryStreamExt as _;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};

pub fn person_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/person")
            .service(query_person)
            .service(query_all_person)
            .service(update_person)
            .service(reset_xp_5_starts)
            .service(reset_xp)
            .service(update_backpack)
            .service(update_stash)
            .service(update_group_type)
            .service(insert_all_person_backpack)
            .service(insert_selected_person_backpack)
            .service(delete_item_list)
            .service(delete_selected_person_item_list)
            .service(update_all_soldier_group)
            .service(update_selected_soldier_group)
            .service(download_person)
            .service(upload_person),
    );
}

#[instrument]
#[get("/query/{id}")]
async fn query_person(config: web::Data<AppData>, id: web::Path<(u64,)>) -> impl Responder {
    info!("");
    let res = extract_person(id.into_inner().0, &config.rwr_profile_folder_path);

    match res {
        Ok(person) => {
            info!("query res, person: {:?}", person);
            HttpResponse::Ok().json(person)
        }
        Err(err) => {
            error!("extract err: {:?}", err);
            HttpResponse::NotFound().json(ResponseJson::default().set_err_msg(&err.to_string()))
        }
    }
}

#[instrument]
#[get("/query_all")]
async fn query_all_person(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let cloned_folder_path = config.rwr_profile_folder_path.clone();
    return match async_extract_all_person_and_profiles(cloned_folder_path).await {
        Ok(all_person_and_profiles_list) => {
            info!("query all peron res {:?}", all_person_and_profiles_list);
            HttpResponse::Ok().json(all_person_and_profiles_list)
        }
        Err(err) => {
            error!("query all person error: {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("query all person error"))
        }
    };
}

#[instrument]
#[post("/update/{id}")]
async fn update_person(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<UpdatePersonReq>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    info!(
        "update person success, query id: {:?}, source {:?}",
        query_id, source
    );

    HttpResponse::Ok().json(ResponseJson::default())
}

#[instrument]
#[post("/reset_xp_5_stars/{id}")]
async fn reset_xp_5_starts(config: web::Data<AppData>, id: web::Path<(u64,)>) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    return match source {
        Ok(person) => {
            let new_person = Person {
                max_authority_reached: 11.098661,
                authority: 11.098661,
                ..person
            };

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => HttpResponse::Ok()
                    .json(ResponseJson::default().set_successful_msg("update stash successful")),
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("save person error"))
        }
    };
}

#[instrument]
#[post("/reset_xp/{id}")]
async fn reset_xp(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<ResetXpReq>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);
    let data: ResetXpReq = data.into_inner();

    return match source {
        Ok(person) => {
            let new_person = Person {
                max_authority_reached: data.authority,
                authority: data.authority,
                ..person
            };

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => HttpResponse::Ok()
                    .json(ResponseJson::default().set_successful_msg("update stash successful")),
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("save person error"))
        }
    };
}

#[instrument]
#[post("/update_backpack/{id}")]
async fn update_backpack(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<Vec<ItemGroupTag>>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    return match source {
        Ok(person) => {
            let new_person = Person {
                backpack_item_list: data.into_inner(),
                ..person
            };

            if new_person.backpack_item_list.len() > new_person.backpack_hard_capacity.into() {
                error!("backpack item over 255");
                return HttpResponse::BadRequest()
                    .json(ResponseJson::default().set_err_msg("backpack overload 255"));
            }

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => HttpResponse::Ok()
                    .json(ResponseJson::default().set_successful_msg("update stash successful")),
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("merge person error"))
        }
    };
}

#[instrument]
#[post("/update_stash/{id}")]
async fn update_stash(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<Vec<ItemGroupTag>>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    return match source {
        Ok(person) => {
            let new_person = Person {
                stash_item_list: data.into_inner(),
                ..person
            };

            if new_person.stash_item_list.len() > new_person.stash_hard_capacity.into() {
                error!("stash item over 300");
                return HttpResponse::BadRequest()
                    .json(ResponseJson::default().set_err_msg("stash overload 300"));
            }

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => HttpResponse::Ok()
                    .json(ResponseJson::default().set_successful_msg("update stash successful")),
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("merge person error"))
        }
    };
}

#[instrument]
#[post("/update_group_type/{id}")]
async fn update_group_type(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<GroupInfo>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    return match source {
        Ok(person) => {
            let new_person = Person {
                soldier_group_name: data.into_inner().group_type,
                ..person
            };

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => HttpResponse::Ok().json(
                    ResponseJson::default().set_successful_msg("update group type successful"),
                ),
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("merge person error"))
        }
    };
}

#[instrument]
#[post("/insert_all_person_backpack")]
async fn insert_all_person_backpack(
    config: web::Data<AppData>,
    data: web::Json<Vec<ItemGroupTag>>,
) -> impl Responder {
    info!("");

    let insert_backpack_item_list = data.into_inner();

    let folder_clone = config.rwr_profile_folder_path.clone();

    return match async_extract_all_person(folder_clone).await {
        Ok(all_person_list) => {
            match insert_person_list_backpack_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &insert_backpack_item_list,
            )
            .await
            {
                Ok(()) => {
                    info!(
                        "inser all person backpack success, backpack_item_list: {:?}",
                        insert_backpack_item_list
                    );
                    HttpResponse::Ok().json(
                        ResponseJson::default()
                            .set_successful_msg("update all person backpack successful"),
                    )
                }
                Err(err) => {
                    error!("insert all person backpack to file person error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save person error"))
                }
            }
        }
        Err(err) => {
            error!("update all person backpack error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("update all person backpack error"))
        }
    };
}

#[instrument]
#[post("/insert_selected_person_backpack")]
async fn insert_selected_person_backpack(
    config: web::Data<AppData>,
    data: web::Json<InsertSelectedPersonBackpackReq>,
) -> impl Responder {
    info!("");

    let insert_data_pre = data.into_inner();

    let insert_backpack_item_list = insert_data_pre.backpack_item_list;
    let profile_id_list = insert_data_pre.profile_id_list;

    let folder_clone = config.rwr_profile_folder_path.clone();

    return match async_extract_selected_person(folder_clone, profile_id_list).await {
        Ok(all_person_list) => {
            match insert_person_list_backpack_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &insert_backpack_item_list,
            )
            .await
            {
                Ok(()) => {
                    info!(
                        "inser selected person backpack success, backpack_item_list: {:?}",
                        insert_backpack_item_list
                    );
                    HttpResponse::Ok().json(
                        ResponseJson::default()
                            .set_successful_msg("update person person backpack successful"),
                    )
                }
                Err(err) => {
                    error!(
                        "insert selected person backpack to file person error {:?}",
                        err
                    );
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save selected person error"))
                }
            }
        }
        Err(err) => {
            error!("update selected person backpack error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("update selected person backpack error"))
        }
    };
}

#[instrument]
#[post("/delete_item_list")]
async fn delete_item_list(
    config: web::Data<AppData>,
    data: web::Json<Vec<String>>,
) -> impl Responder {
    info!("");

    let item_list: Vec<String> = data.into_inner();

    let folder_clone = config.rwr_profile_folder_path.clone();

    return match async_extract_all_person(folder_clone).await {
        Ok(all_person_list) => {
            return match delete_person_item_list_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &item_list,
            )
            .await
            {
                Ok(()) => {
                    info!("delete item list success, item_list: {:?}", item_list);
                    HttpResponse::Ok().json(
                        ResponseJson::default().set_successful_msg("delete item list successful"),
                    )
                }
                Err(err) => {
                    error!("delete item list error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save selected person error"))
                }
            }
        }
        Err(err) => {
            error!("delete item list to file person error {:?}", err);

            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("delete item list to person error"))
        }
    };
}

#[instrument]
#[post("/delete_selected_person_item_list")]
async fn delete_selected_person_item_list(
    config: web::Data<AppData>,
    data: web::Json<DeleteSelectedPersonItemListReq>,
) -> impl Responder {
    info!("");

    let delete_data_pre = data.into_inner();

    let item_list = delete_data_pre.item_list;
    let profile_id_list = delete_data_pre.profile_id_list;

    let folder_clone = config.rwr_profile_folder_path.clone();

    return match async_extract_selected_person(folder_clone, profile_id_list).await {
        Ok(all_person_list) => {
            return match delete_person_item_list_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &item_list,
            )
            .await
            {
                Ok(()) => {
                    info!("delete item list success, item_list: {:?}", item_list);
                    HttpResponse::Ok().json(
                        ResponseJson::default().set_successful_msg("delete item list successful"),
                    )
                }
                Err(err) => {
                    error!("delete item list error {:?}", err);
                    HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("save selected person error"))
                }
            }
        }
        Err(err) => {
            error!("delete item list to file person error {:?}", err);

            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("delete item list to person error"))
        }
    };
}

#[instrument]
#[post("/update_all_person_soldier_group")]
async fn update_all_soldier_group(
    config: web::Data<AppData>,
    data: web::Json<UpdateAllPersonSoldierGroupReq>,
) -> impl Responder {
    info!("");

    let cloned_folder_path = config.rwr_profile_folder_path.clone();
    let data: UpdateAllPersonSoldierGroupReq = data.into_inner();

    return match async_extract_all_person(cloned_folder_path).await {
        Ok(all_person_list) => {
            match update_person_list_soldider_group_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &data.group,
                data.cost,
            )
            .await
            {
                Ok(err_profile_id_list) => HttpResponse::Ok().json(UpdatePersonSoldierGroupRes {
                    error_profile_list: err_profile_id_list,
                }),
                Err(err) => {
                    error!("update all person soldider group to file error {:?}", err);
                    HttpResponse::BadRequest().json(
                        ResponseJson::default()
                            .set_err_msg("update all person soldider group to file error"),
                    )
                }
            }
        }
        Err(err) => {
            error!("update all person soldider group error {:?}", err);

            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("update all person soldider group error"))
        }
    };
}

#[instrument]
#[post("/update_selected_person_soldier_group")]
async fn update_selected_soldier_group(
    config: web::Data<AppData>,
    data: web::Json<UpdateSelectedPersonSoldierGroupReq>,
) -> impl Responder {
    info!("");

    let cloned_folder_path = config.rwr_profile_folder_path.clone();
    let data: UpdateSelectedPersonSoldierGroupReq = data.into_inner();

    return match async_extract_selected_person(cloned_folder_path, data.profile_id_list).await {
        Ok(all_person_list) => {
            match update_person_list_soldider_group_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &data.group,
                data.cost,
            )
            .await
            {
                Ok(err_profile_id_list) => HttpResponse::Ok().json(UpdatePersonSoldierGroupRes {
                    error_profile_list: err_profile_id_list,
                }),
                Err(err) => {
                    error!(
                        "update selected person soldider group to file error {:?}",
                        err
                    );
                    HttpResponse::BadRequest().json(
                        ResponseJson::default()
                            .set_err_msg("update selected person soldider group to file error"),
                    )
                }
            }
        }
        Err(err) => {
            error!("update selected person soldider group error {:?}", err);

            HttpResponse::BadRequest().json(
                ResponseJson::default().set_err_msg("update selected person soldider group error"),
            )
        }
    };
}

#[instrument]
#[get("/download/{id}")]
async fn download_person(config: web::Data<AppData>, id: web::Path<(u64,)>) -> Result<NamedFile> {
    info!("");

    let id: u64 = id.into_inner().0;
    let path = format!("{}/{}.person", &config.rwr_profile_folder_path, id);

    Ok(NamedFile::open_async(path)
        .await
        .map_err(|err| {
            let err_msg = format!("download {} person error: {}", id, err.to_string());
            error!("{}", err_msg);

            let custom_err = ResponseJson::default().set_err_msg(&err_msg);

            HttpResponse::BadRequest().json(custom_err)
        })
        .map_err(|e| {
            error!("download {} person file error: {:?}", id, e);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("download person error"))
        })
        .unwrap())
}

#[post("/upload/{id}")]
async fn upload_person(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let id: u64 = id.into_inner().0;

    info!("in upload person service, id: {}", id);

    let mut temp_file_name = Arc::new(Mutex::new(String::new()));

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = content_disposition.get_filename().unwrap();
        // .map_or_else(|| "temp.person");

        let mut outer_file_name = Arc::clone(&temp_file_name);
        let mut outer_file_name = outer_file_name.lock().unwrap();
        *outer_file_name = String::from(filename);

        let filepath = format!("{}/{}", &config.server_upload_temp_folder_path, &filename);
        info!("filepath: {}", filepath);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    // while let Some(mut field) = payload.try_next().map_err(|err| {
    //     let err_msg = format!("read {} person upload file payload try_next error: {}", id, err.to_string());
    //     error!("{}", err_msg);

    //     let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //     actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    // }).await? {
    //     // A multipart/form-data stream has to contain `content_disposition`
    //     let content_disposition = field.content_disposition();

    //     if let Some(cd) = content_disposition {
    //         info!("content_disposition: {:?}", cd);

    //         let filename = cd.get_filename().unwrap_or_else(|| "temp.person");

    //         let mut outer_file_name = Arc::clone(&temp_file_name);
    //         let mut outer_file_name = outer_file_name.lock().unwrap();
    //         *outer_file_name = String::from(filename);

    //         let filepath = format!("{}/{}", &config.server_upload_temp_folder_path, &filename);
    //         info!("filepath: {}", filepath);

    //         // File::create is blocking operation, use threadpool
    //         let mut f = web::block(|| std::fs::File::create(filepath)).map_err(|err| {
    //             let err_msg = format!("create {} person file by upload error: {}", id, err.to_string());
    //             error!("{}", err_msg);

    //             let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //             actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //         }).await?;

    //         // Field in turn is stream of *Bytes* object
    //         while let Some(chunk) = field.try_next().map_err(|err| {
    //             let err_msg = format!("read {} person upload file field try_next error: {}", id, err.to_string());
    //             error!("{}", err_msg);

    //             let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //             actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //         }).await? {
    //             // filesystem operations are blocking, we have to use threadpool
    //             f = web::block(move || f.write_all(&chunk).map(|_| f)).map_err(|err| {
    //                 let err_msg = format!("write {} person upload file chunk error: {}", id, err.to_string());
    //                 error!("{}", err_msg);

    //                 let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //                 actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //             }).await?;
    //         }
    //     }
    // }

    let temp_file_name = temp_file_name.lock().unwrap();
    info!("Ready to validate filename: {}", &temp_file_name);

    return match extract_person(id, &config.server_upload_temp_folder_path) {
        Ok(person) => {
            if person.backpack_item_list.len() > person.backpack_hard_capacity.into() {
                let custom_err = ResponseJson::default().set_err_msg("person backpack over 255");

                return Ok(HttpResponse::BadRequest().json(custom_err));
            }

            if person.stash_item_list.len() > person.stash_hard_capacity.into() {
                let custom_err = ResponseJson::default().set_err_msg("person stash over 300");

                return Ok(HttpResponse::BadRequest().json(custom_err));
            }

            let from_path = format!(
                "{}/{}",
                &config.server_upload_temp_folder_path, temp_file_name
            );
            let target_path = format!("{}/{}", &config.rwr_profile_folder_path, temp_file_name);

            return match std::fs::copy(from_path, target_path) {
                Ok(_) => Ok(HttpResponse::Ok()
                    .json(
                        ResponseJson::default()
                            .set_successful_msg("upload & replace person success"),
                    )
                    .into()),
                Err(err) => {
                    let err_msg = format!("extract {} person error: {}", id, err.to_string());
                    error!("{}", err);

                    let custom_err = ResponseJson::default().set_err_msg(&err_msg);

                    Ok(HttpResponse::BadRequest().json(custom_err))
                }
            };
        }
        Err(err) => {
            let err_msg = format!("extract {} person error: {}", id, err.to_string());
            error!("{}", err);

            let custom_err = ResponseJson::default().set_err_msg(&err_msg);

            Ok(HttpResponse::BadRequest().json(custom_err))
        }
    };
}

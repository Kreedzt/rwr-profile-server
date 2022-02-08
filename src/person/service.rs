use super::model::{InsertSelectedPersonBackpackReq, Person, StashItemTag, UpdatePersonReq};
use crate::constant::{MAX_BACKPACK_LEN, MAX_STASH_LEN};
use crate::model::ResponseJson;
use crate::person::extract::{extract_all_person, extract_all_person_and_profiles, extract_person};
use crate::person::model::GroupInfo;
use crate::person::save::{
    insert_all_person_backpack_to_file, insert_selected_person_backpack_to_file,
    save_person_to_file,
};
use crate::{AppData};
use actix_web::{get, post, web, Result, HttpResponse, Responder};
use tracing::instrument;
use tracing::log::{error, info};
use actix_files::{NamedFile, HttpRange};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;

pub fn person_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/person")
            .service(query_person)
            .service(query_all_person)
            .service(update_person)
            .service(reset_xp_5_starts)
            .service(update_backpack)
            .service(update_stash)
            .service(update_group_type)
            .service(insert_all_person_backpack)
            .service(insert_selected_person_backpack)
            .service(download_person)
            .service(upload_person)
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
#[post("/update/{id}")]
async fn update_person(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<UpdatePersonReq>,
) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    info!("update person success, query id: {:?}, source {:?}", query_id, source);

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
#[post("/update_backpack/{id}")]
async fn update_backpack(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    data: web::Json<Vec<StashItemTag>>,
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

            if new_person.backpack_item_list.len() > MAX_BACKPACK_LEN.into() {
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
    data: web::Json<Vec<StashItemTag>>,
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

            if new_person.stash_item_list.len() > MAX_STASH_LEN.into() {
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
    data: web::Json<Vec<StashItemTag>>,
) -> impl Responder {
    info!("");

    let insert_backpack_item_list = data.into_inner();

    return match extract_all_person(&config.rwr_profile_folder_path) {
        Ok(all_person_list) => {
            match insert_all_person_backpack_to_file(
                &config.rwr_profile_folder_path,
                &all_person_list,
                &insert_backpack_item_list,
            ) {
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

    let backpack_list = insert_data_pre.backpack_item_list;
    let profile_id_list = insert_data_pre.profile_id_list;

    return match insert_selected_person_backpack_to_file(
        &config.rwr_profile_folder_path,
        &profile_id_list,
        &backpack_list,
    ) {
        Ok(res) => {
            info!("insert selected person backpack success, profile_id_list: {:?}, backpack_list: {:?}", profile_id_list, backpack_list);
            HttpResponse::Ok().json(
                ResponseJson::default()
                    .set_successful_msg("insert_selected person backpack to file success"),
            )
        }
        Err(err) => {
            error!("insert selected person backpack error: {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("query all person error"))
        }
    };
}

#[instrument]
#[get("/query_all")]
async fn query_all_person(config: web::Data<AppData>) -> impl Responder {
    info!("");
    return match extract_all_person_and_profiles(&config.rwr_profile_folder_path) {
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
#[get("/download/{id}")]
async fn download_person(config: web::Data<AppData>, id: web::Path<(u64,)>) -> Result<NamedFile> {
    info!("");

    let id: u64 = id.into_inner().0;
    let path = format!("{}/{}.person", &config.rwr_profile_folder_path, id);

    Ok(NamedFile::open(path).map_err(|err| {
        let err_msg = format!("download {} person error: {}", id, err.to_string());
        error!("{}", err_msg);

        let custom_err = ResponseJson::default().set_err_msg(&err_msg);

        HttpResponse::BadRequest()
            .json(custom_err)
    })?)
}


#[instrument]
#[post("/upload/{id}")]
async fn upload_person(config: web::Data<AppData>) -> impl Responder {
    info!("");

    // while let Some(mut field) = payload.try_next().await? {
    //     // A multipart/form-data stream has to contain `content_disposition`
    //     let content_disposition = field.content_disposition();

    //     let filename = content_disposition
    //         .get_filename().map_or_else(|| "temp-person.person");
    //     let filepath = format!("./tmp/{}", filename);

    //     // File::create is blocking operation, use threadpool
    //     let mut f = web::block(|| std::fs::File::create(filepath)).await??;

    //     // Field in turn is stream of *Bytes* object
    //     while let Some(chunk) = field.try_next().await? {
    //         // filesystem operations are blocking, we have to use threadpool
    //         f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
    //     }
    // }

    HttpResponse::Ok()
}

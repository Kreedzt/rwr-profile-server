// SPDX-License-Identifier: GPL-3.0-only
use crate::profile::model::QueryAllCacheV2Res;
use crate::{model::ResponseJson, profile::extract::extract_profile, AppData};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder, Result};
use futures_util::{TryFutureExt, TryStreamExt as _};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing::{error, info, instrument};

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(query_profile)
            .service(query_all_cache)
            .service(query_all_cache_v2)
            .service(update_profile)
            .service(download_profile)
            .service(upload_profile),
    );
}

#[instrument]
#[get("/query/{id}")]
async fn query_profile(config: web::Data<AppData>, id: web::Path<(u64,)>) -> impl Responder {
    info!("");

    let res = extract_profile(id.into_inner().0, &config.rwr_profile_folder_path);

    match res {
        Ok(profile) => {
            info!("query res, profile: {:?}", profile);
            HttpResponse::Ok().json(profile)
        }
        Err(err) => {
            error!("extract error {:?}", err);
            HttpResponse::NotFound().json(ResponseJson::default().set_err_msg(&err.to_string()))
        }
    }
}

#[instrument]
#[get("/query_all_cache")]
async fn query_all_cache(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let data = config.snapshot_data.lock().await;

    let v = data.clone();

    info!("query all cache successful");
    HttpResponse::Ok().json(v)
}

#[instrument]
#[get("/query_all_cache_v2")]
async fn query_all_cache_v2(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let list_str = config.snapshot_str.lock().await;
    let snapshot_time = config.snapshot_time.lock().await;

    let res = QueryAllCacheV2Res {
        all_person_list_str: list_str.clone(),
        snapshot_time: snapshot_time.clone(),
    };

    info!("query all cache successful");
    HttpResponse::Ok().json(res)
}

// TODO
#[instrument]
#[post("/update_all_profile_squad_tag")]
async fn update_all_profile_squad_tag(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let cloned_folder_path = config.rwr_profile_folder_path.clone();

    HttpResponse::Ok()
}

// TODO
#[instrument]
#[post("/update_selected_profile_squad_tag")]
async fn update_selected_profile_squad_tag(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let cloned_folder_path = config.rwr_profile_folder_path.clone();

    HttpResponse::Ok()
}

#[instrument]
#[post("/update")]
async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("update profile")
}

#[instrument]
#[get("/download/{id}")]
async fn download_profile(config: web::Data<AppData>, id: web::Path<(u64,)>) -> Result<NamedFile> {
    info!("");

    let id: u64 = id.into_inner().0;
    let path = format!("{}/{}.profile", &config.rwr_profile_folder_path, id);

    Ok(NamedFile::open_async(path)
        .await
        .map_err(|err| {
            let err_msg = format!("download {} profile error: {}", id, err.to_string());
            error!("{}", err_msg);

            let custom_err = ResponseJson::default().set_err_msg(&err_msg);

            HttpResponse::BadRequest().json(custom_err)
        })
        .unwrap())
}

#[post("/upload/{id}")]
async fn upload_profile(
    config: web::Data<AppData>,
    id: web::Path<(u64,)>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let id: u64 = id.into_inner().0;

    info!("in upload profile service, id: {}", id);

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
    //     let err_msg = format!("read {} profile upload file payload try_next error: {}", id, err.to_string());
    //     error!("{}", err_msg);

    //     let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //     actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    // }).await? {
    //     // A multipart/form-data stream has to contain `content_disposition`
    //     let content_disposition = field.content_disposition();

    //     if let Some(cd) = content_disposition {
    //         info!("content_disposition: {:?}", cd);

    //         let filename = cd.get_filename().unwrap_or_else(|| "temp.profile");

    //         let mut outer_file_name = Arc::clone(&temp_file_name);
    //         let mut outer_file_name = outer_file_name.lock().unwrap();
    //         *outer_file_name = String::from(filename);

    //         let filepath = format!("{}/{}", &config.server_upload_temp_folder_path, &filename);
    //         info!("filepath: {}", filepath);

    //         // File::create is blocking operation, use threadpool
    //         let mut f = web::block(|| std::fs::File::create(filepath)).map_err(|err| {
    //             let err_msg = format!("create {} profile file by upload error: {}", id, err.to_string());
    //             error!("{}", err_msg);

    //             let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //             actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //         }).await?;

    //         // Field in turn is stream of *Bytes* object
    //         while let Some(chunk) = field.try_next().map_err(|err| {
    //             let err_msg = format!("read {} profile upload file field try_next error: {}", id, err.to_string());
    //             error!("{}", err_msg);

    //             let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //             actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //         }).await? {
    //             // filesystem operations are blocking, we have to use threadpool
    //             f = web::block(move || f.write_all(&chunk).map(|_| f)).map_err(|err| {
    //                 let err_msg = format!("write {} profile upload file chunk error: {}", id, err.to_string());
    //                 error!("{}", err_msg);

    //                 let custom_err = ResponseJson::default().set_err_msg(&err_msg);

    //                 actix_web::error::ErrorBadRequest(serde_json::to_string(&custom_err).unwrap())
    //             }).await?;
    //         }
    //     }
    // }

    let temp_file_name = temp_file_name.lock().unwrap();
    info!("Ready to validate filename: {}", &temp_file_name);

    let from_path = format!(
        "{}/{}",
        &config.server_upload_temp_folder_path, temp_file_name
    );
    let target_path = format!("{}/{}", &config.rwr_profile_folder_path, temp_file_name);

    return match std::fs::copy(from_path, target_path) {
        Ok(_) => Ok(HttpResponse::Ok()
            .json(ResponseJson::default().set_successful_msg("upload & replace profile success"))
            .into()),
        Err(err) => {
            let err_msg = format!("extract {} person error: {}", id, err.to_string());
            error!("{}", err);

            let custom_err = ResponseJson::default().set_err_msg(&err_msg);

            Ok(HttpResponse::BadRequest().json(custom_err))
        }
    };
}

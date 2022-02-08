use crate::{model::ResponseJson, profile::extract::extract_profile, AppData};
use actix_web::{get, post, web, Result, HttpResponse, Responder};
use tracing::{error, info, instrument};
use actix_files::{NamedFile};

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(query_profile)
            .service(update_profile)
            .service(download_profile),
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

    Ok(NamedFile::open(path).map_err(|err| {
        let err_msg = format!("download {} profile error: {}", id, err.to_string());
        error!("{}", err_msg);

        let custom_err = ResponseJson::default().set_err_msg(&err_msg);

        HttpResponse::BadRequest()
            .json(custom_err)
    })?)
}

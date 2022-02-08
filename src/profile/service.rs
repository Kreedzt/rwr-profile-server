use crate::{model::ResponseJson, profile::extract::extract_profile, AppData};
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{error, info, instrument};

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(query_profile)
            .service(update_profile),
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

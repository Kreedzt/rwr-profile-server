use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::instrument;
use tracing::log::{info, error};
use crate::Config;
use crate::person::extract::extract_person;

pub fn person_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/person")
            .service(query_person)
            .service(update_person)
            .service(reset_xp_5_starts)
            .service(update_stash)
    );
}

#[instrument]
#[get("/query/{id}")]
async fn query_person(config: web::Data<Config>,id: web::Path<(u64,)>) -> impl Responder {
    let res = extract_person(id.into_inner().0, &config.rwr_profile_folder_path);

    match res {
        Ok(person) => {
            info!("query res, person: {:?}", person);
            HttpResponse::Ok().json(person);
        },
        Err(err) => {
            error!("extract err: {:?}", err);
            HttpResponse::NotFound().body("extract err");
        }
    }
}

#[instrument]
#[post("/update")]
async fn update_person() -> impl Responder {
    HttpResponse::Ok().body("update person")
}

#[instrument]
#[post("/reset_xp_5_stars")]
async fn reset_xp_5_starts() -> impl Responder {
    HttpResponse::Ok().body("reset xp 5 starts")
}

#[instrument]
#[post("/update_stash")]
async fn update_stash() -> impl Responder {
    HttpResponse::Ok().body("update stash")
}

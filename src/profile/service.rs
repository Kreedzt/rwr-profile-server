use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::instrument;
use tracing::log::info;
use crate::Config;

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(query_profile)
            .service(update_profile)
            .service(reset_xp_5_starts)
    );
}

#[instrument]
#[get("/query/{id}")]
async fn query_profile(config: web::Data<Config>,id: web::Path<(u64,)>) -> impl Responder {
    HttpResponse::Ok().body("query profile")
}

#[instrument]
#[post("/update")]
async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("update profile")
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

use crate::AppData;
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::instrument;

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
    HttpResponse::Ok().body("query profile")
}

#[instrument]
#[post("/update")]
async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("update profile")
}

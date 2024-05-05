use actix_web::{get, web, HttpResponse, Responder};
use tracing::{info, instrument};

pub fn ping_config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}

#[instrument]
#[get("/ping")]
async fn ping() -> impl Responder {
    info!("");

    HttpResponse::Ok().body("pong!")
}

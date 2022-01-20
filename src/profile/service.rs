use actix_web::{HttpResponse, Responder, get, post, web};

pub fn profile_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .service(get_profile)
            .service(update_profile)
    );
}

#[get("/query")]
async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("query profile")
}

#[post("/update")]
async fn update_profile() -> impl Responder {
    HttpResponse::Ok().body("update profile")
}
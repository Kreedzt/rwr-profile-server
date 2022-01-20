use actix_web::{HttpResponse, Responder, get};

#[get("/info")]
pub async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("user info!")
}

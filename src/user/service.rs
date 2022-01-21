use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::{error, instrument};
use tracing::log::info;
use crate::Config;
use crate::user::model::{LoginReq, RegisterReq};
use crate::user::utils::register_user;

pub fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(register)
            .service(login)
            .service(get_user)
            .service(get_all_user)
    );
}

#[instrument]
#[post("/register")]
async fn register(config: web::Data<Config>, user: web::Json<RegisterReq>) -> impl Responder {
    info!("user/register, config: {:?}, {:?}", config, user);
    let res = register_user(&user.username, &user.password, &config.server_data_folder_path);

    match res {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            error!("[user]: register service, error: {:?}", err);
            HttpResponse::BadRequest().body("err")
        },
    }
}

#[instrument]
#[post("/login")]
async fn login(config: web::Data<Config>, info: web::Json<LoginReq>) -> impl Responder {
    let profile_folder = &config.rwr_profile_folder_path;
    println!("profile_folder: {}", profile_folder);
    println!("info: {:?}", info);
    HttpResponse::Ok().body("OK")
}

#[instrument]
#[get("/query/{id}")]
async fn get_user(id: web::Path<(u64,)>) -> impl Responder {
    HttpResponse::Ok().body("user info!")
}

#[instrument]
#[get("/query_all")]
async fn get_all_user(config: web::Data<Config>) -> impl Responder {
    HttpResponse::Ok().body("all user info!")
}

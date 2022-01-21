use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::{error, instrument};
use tracing::log::{info, warn};
use crate::{AppData, Config};
use crate::user::model::{LoginReq, RegisterReq, User};
use crate::user::utils::{get_user_info, get_user_json_data, register_user, update_user_list, validate_user};

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
async fn register(config: web::Data<AppData>, user: web::Json<RegisterReq>) -> impl Responder {
    info!("");
    config.user_json_lock.lock();

    info!("call register_us fn");
    let res = register_user(&user.username, &user.password, &config);

    match res {
        Ok(id) => {
            return match get_user_json_data(&config.server_data_folder_path) {
                Ok(mut user_json_data) => {
                    user_json_data.user_list.push(User {
                        name: user.username.clone(),
                        password: user.password.clone(),
                        user_id: id,
                        admin: 0
                    });

                    match update_user_list(user_json_data.user_list, &config.server_data_folder_path) {
                        Ok(_) => {
                            info!("user: {}, profile_id: {} register successful", user.username, id);
                            HttpResponse::Ok().body("register succcessful")
                        },
                        Err(e) => {
                            error!("{:?}", e);
                            HttpResponse::BadRequest().body(e.to_string())
                        }
                    }
                },
                Err(e) => {
                    error!("{:?}", e);
                    HttpResponse::BadRequest().body(e.to_string())
                }
            }
        },
        Err(err) => {
            error!("register, error: {:?}", err);
            HttpResponse::BadRequest().body(err.to_string())
        },
    }
}

#[instrument]
#[post("/login")]
async fn login(config: web::Data<AppData>, info: web::Json<LoginReq>) -> impl Responder {
    config.user_json_lock.lock();

    match validate_user(&info.username, &info.password, &config.server_data_folder_path) {
        Ok(_) => {
            return match get_user_info(&info.username, &config.server_data_folder_path) {
                Ok(res) => {
                    HttpResponse::Ok().json(res)
                },
                Err(e) => {
                    error!("{:?}", e);
                    HttpResponse::BadRequest().body(e.to_string())
                }
            }
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
        }
    }
}

#[instrument]
#[get("/query/{id}")]
async fn get_user(id: web::Path<(u64,)>) -> impl Responder {
    HttpResponse::Ok().body("user info!")
}

#[instrument]
#[get("/query_all")]
async fn get_all_user(config: web::Data<AppData>) -> impl Responder {
    HttpResponse::Ok().body("all user info!")
}

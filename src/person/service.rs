use actix_web::{HttpResponse, Responder, get, post, web};
use tracing::instrument;
use tracing::log::{info, error};
use crate::Config;
use crate::person::extract::extract_person;
use crate::person::save::save_person_to_file;
use super::model::{UpdatePersonReq, StashItemTag, Person};

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
    info!("");
    let res = extract_person(id.into_inner().0, &config.rwr_profile_folder_path);

    match res {
        Ok(person) => {
            info!("query res, person: {:?}", person);
            HttpResponse::Ok().json(person)
        },
        Err(err) => {
            error!("extract err: {:?}", err);
            HttpResponse::NotFound().body("extract err")
        }
    }
}

#[instrument]
#[post("/update/{id}")]
async fn update_person(config: web::Data<Config>, id: web::Path<(u64,)>, data: web::Json<UpdatePersonReq>) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    HttpResponse::Ok().body("update person")
}

#[instrument]
#[post("/reset_xp_5_stars/{id}")]
async fn reset_xp_5_starts(config: web::Data<Config>, id: web::Path<(u64,)>) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    match source {
        Ok(person) => {
            let new_person = Person {
                max_authority_reached: 11.098661,
                authority: 11.098661,
                ..person
            };

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => {
                    HttpResponse::Ok().body("update stash successful")
                },
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::NotFound().body("save person error")
                }
            }
        },
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::NotFound().body("merge person error")
        }
    };

    HttpResponse::Ok().body("reset xp 5 starts")
}

#[instrument]
#[post("/update_stash/{id}")]
async fn update_stash(config: web::Data<Config>, id: web::Path<(u64,)>, data: web::Json<Vec<StashItemTag>>) -> impl Responder {
    info!("");
    let query_id = id.into_inner().0;
    let source = extract_person(query_id, &config.rwr_profile_folder_path);

    match source {
        Ok(person) => {
            let new_person = Person {
                stash_item_list: data.into_inner(),
                ..person
            };

            info!("new_person: {:?}", new_person);

            match save_person_to_file(&config.rwr_profile_folder_path, query_id, &new_person) {
                Ok(_) => {
                    HttpResponse::Ok().body("update stash successful")
                },
                Err(err) => {
                    error!("save person error {:?}", err);
                    HttpResponse::NotFound().body("save person error")
                }
            }
        },
        Err(err) => {
            error!("merge person error {:?}", err);
            HttpResponse::NotFound().body("merge person error")
        }
    };

    HttpResponse::Ok().body("update stash outter successful")
}

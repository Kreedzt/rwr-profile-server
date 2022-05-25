// SPDX-License-Identifier: GPL-3.0-only
use crate::AppData;
use crate::system::extract::get_ranks_data;
use crate::system::model::RankItem;
use crate::{
    model::ResponseJson,
    system::model::QuickItem,
    system::{extract::get_quick_items_data, save::save_quick_items_to_file},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{error, info, instrument};

pub fn system_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/system")
            .service(query_quick_items)
            .service(update_quick_items)
            .service(query_ranks),
    );
}

#[instrument]
#[get("/query_quick_items")]
async fn query_quick_items(config: web::Data<AppData>) -> impl Responder {
    info!("");

    return match get_quick_items_data(&config.server_data_folder_path) {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => {
            error!("query quick items error: {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("query quick items error"))
        }
    };
}

#[instrument]
#[post("/update_quick_items")]
async fn update_quick_items(
    config: web::Data<AppData>,
    data: web::Json<Vec<QuickItem>>,
) -> impl Responder {
    info!("");

    let quick_items: Vec<QuickItem> = data.into_inner();

    return match save_quick_items_to_file(&config.server_data_folder_path, &quick_items) {
        Ok(()) => {
            info!("update quick items successful: {:?}", quick_items);
            HttpResponse::Ok()
                .json(ResponseJson::default().set_successful_msg("update quick items success"))
        }
        Err(err) => {
            error!("update quick items error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default().set_err_msg("update quick items error"))
        }
    };
}

#[instrument]
#[get("/query_ranks")]
async fn query_ranks(config: web::Data<AppData>) -> impl Responder {
    info!("");

    let mut snapshot_ranks = config.snapshot_ranks.lock().await;

    let snapshot_ranks_data: Vec<RankItem> = snapshot_ranks.clone();

    if snapshot_ranks_data.len() == 0 {
        return match get_ranks_data(&config.server_data_folder_path) {
            Ok(data) => {
                *snapshot_ranks = data.clone();

                HttpResponse::Ok().json(data.clone())
            },
            Err(err) => {
                error!("query ranks error: {:?}", err);
                HttpResponse::BadRequest()
                    .json(ResponseJson::default().set_err_msg("query ranks error"))
            }
        }
    }

    return HttpResponse::Ok().json(snapshot_ranks_data);
}

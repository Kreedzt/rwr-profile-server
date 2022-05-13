use crate::{model::ResponseJson, system::{extract::get_quick_items_data, save::save_quick_items_to_file}, system::model::QuickItem};
use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::{error, info, instrument};
use crate::AppData;

pub fn system_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/system")
            .service(query_quick_items)
            .service(update_quick_items)
    );
}

#[instrument]
#[get("/query_quick_items")]
async fn query_quick_items(config:web::Data<AppData>) -> impl Responder {
    info!("");

    return match get_quick_items_data(&config.server_data_folder_path) {
        Ok(data) => {
            HttpResponse::Ok().json(
                data
            )
        }
        Err(err) => {
            error!("query quick items error {:?}", err);
            HttpResponse::BadRequest()
                        .json(ResponseJson::default().set_err_msg("query quick items error"))
        }
    }
}

#[instrument]
#[post("/update_quick_items")]
async fn update_quick_items(
    config: web::Data<AppData>,
    data: web::Json<Vec<QuickItem>>) -> impl Responder {
    info!("");

    let quick_items: Vec<QuickItem> = data.into_inner();

    return match save_quick_items_to_file(&config.server_data_folder_path, &quick_items) {
        Ok(()) => {
            info!("update quick items successful: {:?}", quick_items);
            HttpResponse::Ok()
                .json(ResponseJson
                ::default()
                .set_successful_msg("update quick items success"))
        }
        Err(err) => {
            error!("update quick items error {:?}", err);
            HttpResponse::BadRequest()
                .json(ResponseJson::default()
                .set_err_msg("update quick items error"))
        }
    }
}

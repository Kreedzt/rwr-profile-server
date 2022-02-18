use crate::model::{AppData, Config};
use crate::person::service::person_config;
use crate::profile::service::profile_config;
use crate::user::service::user_config;
use actix_web::{web, App, HttpServer};
use anyhow::{Error, Result};
use std::sync::Mutex;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{filter::LevelFilter, prelude::*};
use tokio;

mod constant;
mod init;
mod model;
mod person;
mod profile;
mod user;

// #[actix_web::main]
#[tokio::main]
async fn main() -> Result<()> {
    let config = init::init_config()?;

    let server_log_folder_path = config.server_log_folder_path.clone();

    let app_data = web::Data::new(AppData {
        server_data_folder_path: config.server_data_folder_path,
        rwr_profile_folder_path: config.rwr_profile_folder_path,
        server_log_folder_path: config.server_log_folder_path,
        server_upload_temp_folder_path: config.server_upload_temp_folder_path,
        user_json_lock: Mutex::new(0),
    });

    let file_appender = rolling::daily(&server_log_folder_path, "info.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(LevelFilter::INFO);

    let std_out_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(std_out_layer)
        .with(fmt_layer)
        .init();

    info!("completed reading app_data: {:?}", app_data);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&app_data))
            .configure(user_config)
            .configure(profile_config)
            .configure(person_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .map_err(Error::msg)
}

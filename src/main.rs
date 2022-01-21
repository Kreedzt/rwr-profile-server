use std::sync::{Arc, Mutex};
use actix_web::{App, HttpServer, web};
use tracing::info;
use tracing_subscriber;
use anyhow::{Result, Error};
use crate::model::{AppData, Config};
use crate::person::service::person_config;
use crate::profile::service::profile_config;
use crate::user::service::user_config;

mod user;
mod init;
mod model;
mod profile;
mod person;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = init::init_config()?;
    let user_json_lock = Mutex::new(0);

    let app_data = web::Data::new(AppData {
        server_data_folder_path: config.server_data_folder_path,
        rwr_profile_folder_path: config.rwr_profile_folder_path,
        server_log_folder_path: config.server_log_folder_path,
        user_json_lock: Mutex::new(0)
    });

    tracing_subscriber::FmtSubscriber::builder()
        .init();

    // TODO: why not effect
    // let info_file = rolling::daily(&config.server_log_folder_path, "info");
    // let file_appender = rolling::RollingFileAppender::new(rolling::Rotation::DAILY, &config.server_log_folder_path, "info");
    //
    // let file_appender = tracing_appender::rolling::daily(&config.server_log_folder_path, "info.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // tracing_subscriber::fmt()
    //     .with_writer(non_blocking)
    //     .init();

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

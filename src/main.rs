// SPDX-License-Identifier: GPL-3.0-only
use crate::model::AppData;
use crate::person::{async_extract::async_extract_query_data, service::person_config};
use crate::profile::service::profile_config;
use crate::system::service::system_config;
use crate::user::service::user_config;
use crate::version_update::preupdate::preupdate;
use actix_web::{web, App, HttpServer};
use anyhow::{Error, Result};
use chrono::prelude::*;
use tokio;
use tokio::{
    sync::Mutex,
    time::{interval, Duration},
};
use tracing::{error, info};
use tracing_appender::rolling;
use tracing_subscriber::{filter::LevelFilter, prelude::*};

mod constant;
mod init;
mod model;
mod person;
mod profile;
mod system;
mod user;
mod version_update;
mod ping;

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
        // hourly query_all
        snapshot_data: Mutex::new(vec![]),
        snapshot_str: Mutex::new(String::new()),
        snapshot_time: Mutex::new(String::new()),
        // ranks snapshot
        snapshot_ranks: Mutex::new(vec![]),
    });

    let std_out_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(LevelFilter::INFO);

    if !server_log_folder_path.is_empty() {
        let file_appender = rolling::daily(&server_log_folder_path, "info.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .with_writer(non_blocking)
            .with_filter(LevelFilter::INFO);

        tracing_subscriber::registry()
            .with(std_out_layer)
            .with(fmt_layer)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(std_out_layer)
            .init();
    }

    info!("completed reading app_data: {:?}", app_data);

    let app_data_c = app_data.clone();

    // update file version
    let folder_path = app_data_c.rwr_profile_folder_path.clone();
    preupdate(folder_path).await?;

    if config.server_hourly_request {
        tokio::task::spawn(async move {
            // 1 hour interval
            let mut interval = interval(Duration::from_secs(60 * 60));

            loop {
                interval.tick().await;

                let folder_path = app_data_c.rwr_profile_folder_path.clone();

                match async_extract_query_data(folder_path).await {
                    Ok(all_person_and_profiles_list) => {
                        info!("query all peron res {:?}", all_person_and_profiles_list);

                        let mut snapshot_data = app_data_c.snapshot_data.lock().await;
                        *snapshot_data = all_person_and_profiles_list.clone();

                        let mut snapshot_str = app_data_c.snapshot_str.lock().await;
                        *snapshot_str =
                            serde_json::to_string(&all_person_and_profiles_list).unwrap();

                        // TODO: next release
                        // fs::write("demo2_json.json", &*snapshot_str).await.unwrap();

                        let local = Local::now();
                        let current_time = local.format("%Y-%m-%d %H:%M:%S").to_string();
                        let mut snapshot_time = app_data_c.snapshot_time.lock().await;
                        *snapshot_time = current_time;
                    }
                    Err(err) => {
                        error!("query all person error: {:?}", err);
                    }
                }
            }
        });
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&app_data))
            .configure(user_config)
            .configure(profile_config)
            .configure(person_config)
            .configure(system_config)
            .configure(ping::ping_config)
    })
    .bind(format!("0.0.0.0:{}", config.port))?
    .run()
    .await
    .map_err(Error::msg)
}

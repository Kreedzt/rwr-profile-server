use actix_web::{App, HttpServer};
use tracing::info;
use tracing::instrument::WithSubscriber;
use tracing_subscriber;
use tracing_subscriber::fmt::MakeWriter;
use tracing_appender;
use tracing_appender::rolling;
use tracing_actix_web::TracingLogger;
use anyhow::{Result, Error};
use crate::model::Config;
use crate::profile::service::profile_config;
use crate::user::service::user_config;

mod user;
mod init;
mod model;
mod profile;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = init::init_config()?;

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

    info!("completed reading config: {:?}", config);

    HttpServer::new(move || {
        let config = config.clone();
        App::new()
            .data(config)
            .configure(user_config)
            .configure(profile_config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
        .map_err(Error::msg)
}

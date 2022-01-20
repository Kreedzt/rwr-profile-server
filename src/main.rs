use actix_web::{App, web, HttpServer};
use tracing_subscriber;
use anyhow::{anyhow, Result, Error};
use tracing::info;
use once_cell::sync::Lazy;
use crate::model::Config;
use crate::profile::service::profile_config;
use crate::user::service::{user_config};

mod user;
mod init;
mod model;
mod profile;

// static GLOBAL_DATA: Lazy<()> = Lazy::new(|| {
//     tracing_subscriber::FmtSubscriber::builder()
//         .with_max_level(tracing::Level::DEBUG)
//         .init();
//     ()
// });

#[actix_web::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = init::init_config()?;
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

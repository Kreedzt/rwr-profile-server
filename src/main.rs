use actix_web::{App, web, HttpServer};
use user::service::get_user;

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/user")
                .service(get_user)
        )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

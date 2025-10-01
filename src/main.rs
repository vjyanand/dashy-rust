mod handler;
mod models;

use actix_web::{middleware, web::Data, App, HttpServer};
use env_logger::{Builder, Env};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{env, str::FromStr, time::Duration};

use crate::handler::{apns_register, ok, stat_get, stat_post, stats_get};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::from_env(Env::default())
        .format_timestamp(None) // Optional: remove timestamp
        .write_style(env_logger::WriteStyle::Always) // Force colors
        .init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .expect("PORT must be a number");

    let binding_interface = format!("0.0.0.0:{}", port);
    println!("Listening at {}", binding_interface);
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let options = PgConnectOptions::from_str(&url)
        .expect("Unable to parse")
        .ssl_mode(PgSslMode::Require)
        .application_name("DASHY");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("Unable to create connection pool");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(stat_post)
            .service(stat_get)
            .service(stats_get)
            .service(apns_register)
            .service(ok)
    })
    .bind(binding_interface)?
    .run()
    .await
}

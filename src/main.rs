mod models;

use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use models::{StatPath, StatPayload, Stats, StatsList, StatsPath};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};
use std::{env, str::FromStr, time::Duration};

#[actix_web::post("/stat/{uid}/{id}")]
async fn stat_post(
    pool: web::Data<PgPool>,
    info: web::Path<StatPath>,
    payload: web::Json<StatPayload>,
) -> impl Responder {
    if payload.number.is_none() && payload.string.is_none() {
        return HttpResponse::BadRequest().body("Not found");
    }

    let meta = match serde_json::to_value(&payload) {
        Ok(meta) => meta,
        Err(_) => {
            return HttpResponse::BadRequest().body("No valid meta found");
        }
    };

    let result = sqlx::query_as!(
        Stats,
        r#"UPDATE stats SET meta = $3, updated = NOW() where uid = $1 AND id = $2 RETURNING id, uid, updated, meta"#,
        info.uid,
        info.id,
        meta
    )
    .fetch_optional(&**pool)
    .await;
    let result: Option<Stats> = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("No results"),
    };

    let result = match result {
        Some(result) => result,
        None => return HttpResponse::NotFound().body("Not found"),
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::get("/stats/{uid}")]
async fn stats_get(pool: web::Data<PgPool>, info: web::Path<StatsPath>) -> impl Responder {
    let result = sqlx::query_as!(
        StatsList,
        r#"SELECT id, uid, updated FROM stats WHERE uid = $1"#,
        info.uid
    )
    .fetch_all(&**pool)
    .await;

    let result = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::NotFound().body("Not found"),
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::get("/stat/{uid}/{id}")]
async fn stat_get(pool: web::Data<PgPool>, info: web::Path<StatPath>) -> impl Responder {
    let result = sqlx::query_as!(
        Stats,
        r#"UPDATE stats SET fetched = NOW() where uid = $1 AND id = $2 RETURNING id, uid, meta, updated"#,
        info.uid,
        info.id
    )
    .fetch_optional(&**pool)
    .await;
    let result: Option<Stats> = match result {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("No results"),
    };

    let result = match result {
        Some(result) => result,
        None => return HttpResponse::NotFound().body("Not found"),
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8080"))
        .parse()
        .expect("PORT must be a number");

    let binding_interface = format!("0.0.0.0:{}", port);
    println!("Listening at {}", binding_interface);
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let options = PgConnectOptions::from_str(&url)
        .expect("Unable to parse")
        .ssl_mode(PgSslMode::Allow)
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
            .app_data(Data::new(pool.clone()))
            .service(stat_post)
            .service(stat_get)
            .service(stats_get)
            .service(ok)
    })
    .bind(binding_interface)?
    .run()
    .await
}

#[get("/")]
async fn ok() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

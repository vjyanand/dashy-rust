use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use std::env;

mod postgres;
mod models;

async fn stat(db: web::Data<postgres::PostgresPool>) -> impl Responder {
    let res = web::block(move || {
        let mut conn = db.get().unwrap();
        conn.query_opt("SELECT * from stat", &[])
    })
    .await;

    let res = match res {
        Ok(res) => res,
        Err(_) => return HttpResponse::NotFound().body("Resource Not found"),
    };

    let row = match res {
        Ok(res) => res,
        Err(_) => return HttpResponse::NotFound().body("Resource Not found"),
    };

    match row {
        Some(row) => {
            let val: &str = row.get("val");
            let id:uuid::Uuid = uuid::Uuid::parse_str(row.get("id")).unwrap();
            let uid:uuid::Uuid = uuid::Uuid::parse_str(row.get("uid")).unwrap();
            let stat = models::Stats{ id, uid, val: val.to_string(), updated:1, fetched: 1 };
            return HttpResponse::Ok().json(stat);
        }
        None => {}
    };
    HttpResponse::Ok().body("{}")
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

    let pool = postgres::get_pool();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .app_data(Data::new(pool.clone()))
            .service(web::resource("/stat").route(web::get().to(stat)))
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

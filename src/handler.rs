use actix_web::{
    web::{self},
    HttpResponse, Responder,
};

use log::{info, warn};
use serde_json::Value;
use sqlx::PgPool;

use crate::models::{StatPath, StatPayload, Stats, StatsPath};

#[actix_web::post("/ios/widget/apns/register")]
pub async fn apns_register(pool: web::Data<PgPool>, payload: web::Json<Value>) -> impl Responder {
    let id = uuid::Uuid::new_v4();
    let token = payload.get("token").and_then(|token| token.as_str());
    if let Some(widgets) = payload.get("widgets").and_then(|v| v.as_array()) {
        for widget_obj in widgets {
            let dashy_id = widget_obj.get("dashyId").and_then(|v| v.as_str());
            info!("Registering APNS token: {:?} for widget: {:?}", token, dashy_id);
            let result =
                sqlx::query(r#"INSERT INTO public.apns (id, token, wid) VALUES ($1, $2, $3)"#)
                    .bind(id)
                    .bind(token)
                    .bind(dashy_id)
                    .execute(&**pool)
                    .await;
            match result {
                Ok(_) => log::info!("Registered APNS token: {}", token.unwrap_or("none")),
                Err(e) => {
                    log::error!("Error registering APNS token: {}", e);
                    return HttpResponse::InternalServerError()
                        .body("Error registering APNS token");
                }
            }
        }
    }

    log::info!("Received payload: {:?}", payload.0);
    HttpResponse::Ok().json(payload.0)
}

#[actix_web::post("/stat/{uid}/{id}")]
pub async fn stat_post(
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

    let result = sqlx::query_as(
        r#"UPDATE stats SET meta = $3, updated = NOW() where uid = $1 AND id = $2 RETURNING id, uid, updated, meta, name"#
    )
    .bind(info.uid)
    .bind(info.id)
    .bind(meta).fetch_optional(&**pool).await;

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
pub async fn stats_get(pool: web::Data<PgPool>, info: web::Path<StatsPath>) -> impl Responder {
    log::warn!("Getting stats for uid {}", info.uid);
    let result: Result<Vec<Stats>, sqlx::Error> =
        sqlx::query_as(r#"SELECT id, uid, updated, null as meta, name FROM stats WHERE uid = $1"#)
            .bind(info.uid)
            .fetch_all(&**pool)
            .await;
    if let Err(e) = &result {
        log::error!("Error getting stats for uid {}: {}", info.uid, e);
    }
    if let Ok(result) = result {
        HttpResponse::Ok().json(result)
    } else {
        HttpResponse::NotFound().body("Not found")
    }
}

#[actix_web::get("/stat/{uid}/{id}")]
pub async fn stat_get(pool: web::Data<PgPool>, info: web::Path<StatPath>) -> impl Responder {
    let result: Result<Option<Stats>, sqlx::Error> = sqlx::query_as(
        r#"UPDATE stats SET fetched = NOW() where uid = $1 AND id = $2 RETURNING id, uid, meta, name, updated"#
    ).bind(info.uid)
    .bind(info.id)
    .fetch_optional(&**pool)
    .await;

    let Ok(result) = result else {
        return HttpResponse::InternalServerError().body("No results");
    };

    let Some(result) = result else {
        return HttpResponse::NotFound().body("Not found");
    };

    HttpResponse::Ok().json(result)
}

#[actix_web::get("/")]
pub async fn ok() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

use r2d2_postgres::{postgres::NoTls, r2d2, PostgresConnectionManager};

use std::env;
use std::time::Duration;

pub type PostgresPool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

pub fn get_pool() -> PostgresPool {
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let mgr = PostgresConnectionManager::new(url.as_str().parse().unwrap(), NoTls);
    r2d2::Pool::builder()
        .connection_timeout(Duration::from_secs(5))
        .max_size(1)
        .build(mgr)
        .expect("could not build connection pool")
}

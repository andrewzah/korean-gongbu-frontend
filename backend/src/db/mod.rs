use std::time::Duration;
use std::sync::Arc;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};
use lazy_static::lazy_static;

pub mod user;

type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

lazy_static! {
    static ref DB_POOL: DbPool = init_pool(&database_url());
}

fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("Is DATABASE_URL set?")
}

fn init_pool(db_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);

    let r2d2_pool = r2d2::Pool::builder()
        .max_size(30)
        .idle_timeout(Some(Duration::from_secs(30)))
        .connection_timeout(Duration::from_secs(30))
        .build(manager)
        .expect("Unable to build pool!");

    Arc::new(r2d2_pool)
}

pub fn pool() -> DbPool {
    Arc::clone(&DB_POOL)
}

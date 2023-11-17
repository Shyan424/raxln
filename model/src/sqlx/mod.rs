pub mod entity;
pub mod repository;


use std::sync::OnceLock;

use sqlx::PgPool;

static POOL: OnceLock<PgPool> = OnceLock::new();

pub fn set_datasource(db: PgPool) {
    POOL.set(db).expect("Set Datasource Error");
}

fn get_datasource() -> PgPool {
    POOL.get().expect("Datasource Not Init").clone()
}
use crate::tokio::sync::RwLock;
use moon::Lazy;
use sqlx::postgres::{PgPool, Postgres};
use sqlx::Pool;

pub static POSTGRES: Lazy<RwLock<PgPool>> = Lazy::new(|| {
    RwLock::new(
        Pool::<Postgres>::connect_lazy(
            &dotenvy::var("DATABASE_URL").expect("Database_url must be set"),
        )
        .expect("Failed to connect db"),
    )
});

use sqlx::{PgPool, Postgres};
use dotenvy::dotenv;
use std::env;

pub async fn establish_connection(database_url: &str) -> PgPool {
    dotenv().ok();
    PgPool::connect_lazy(database_url)
        .expect("Falha ao conectar ao banco de dados")
}

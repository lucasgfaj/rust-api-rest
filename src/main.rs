use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::{net::SocketAddr, sync::Arc};
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, FromRow)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL não definida");

    let pool = PgPool::connect(&database_url).await.expect("Erro ao conectar no banco");

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .with_state(Arc::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor rodando em {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn get_users(State(pool): State<Arc<PgPool>>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool.as_ref())
        .await
        .expect("Erro ao buscar usuários");

    Json(users)
}

async fn create_user(
    State(pool): State<Arc<PgPool>>,
    Json(user): Json<User>
) -> Json<User> {
    sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        user.name,
        user.email
    )
    .execute(pool.as_ref())
    .await
    .expect("Erro ao inserir usuário");

    Json(user)
}

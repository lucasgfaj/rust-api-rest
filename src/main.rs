use actix_web::{App, HttpServer, web};
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv; // Importação necessária
use crate::database::connection::establish_connection;
use crate::controllers::user_controller;

mod database;
mod models;
mod services;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Adicione essa linha para carregar o .env
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");
    let pool = establish_connection(&database_url).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_controller::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

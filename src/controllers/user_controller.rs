use actix_web::{web, get, post, put, delete, HttpResponse, Responder};
use crate::services::user_service::UserService;
use crate::models::user::{CreateUser, User};
use sqlx::PgPool;

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match UserService::get_all_users(&pool).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar usuários"),
    }
}

#[get("/users/{id}")]
async fn get_user(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    match UserService::get_user_by_id(&pool, user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Usuário não encontrado"),
    }
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<CreateUser>) -> impl Responder {
    match UserService::create_user(&pool, user.into_inner()).await {
        Ok(new_user) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao criar usuário"),
    }
}

#[put("/users/{id}")]
async fn update_user(pool: web::Data<PgPool>, path: web::Path<i32>, user: web::Json<CreateUser>) -> impl Responder {
    let user_id = path.into_inner();
    match UserService::update_user(&pool, user_id, user.into_inner()).await {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao atualizar usuário"),
    }
}

#[delete("/users/{id}")]
async fn delete_user(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let user_id = path.into_inner();
    match UserService::delete_user(&pool, user_id).await {
        Ok(_) => HttpResponse::Ok().body("Usuário deletado com sucesso"),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao deletar usuário"),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users)
       .service(get_user)
       .service(create_user)
       .service(update_user)
       .service(delete_user);
}

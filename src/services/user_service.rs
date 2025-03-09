use crate::models::user::{User, CreateUser};
use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }

    pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn create_user(pool: &PgPool, user: CreateUser) -> Result<User, sqlx::Error> {
        let new_user = sqlx::query_as::<_, User>(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
        )
        .bind(user.name)
        .bind(user.email)
        .fetch_one(pool)
        .await?;
        Ok(new_user)
    }

    pub async fn update_user(pool: &PgPool, user_id: i32, user: CreateUser) -> Result<User, sqlx::Error> {
        let updated_user = sqlx::query_as::<_, User>(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *",
        )
        .bind(user.name)
        .bind(user.email)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(updated_user)
    }

    pub async fn delete_user(pool: &PgPool, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

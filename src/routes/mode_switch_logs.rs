use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

// CREATE TABLE mode_switch_logs (
//     id SERIAL PRIMARY KEY,
//     user_id INT REFERENCES users(id) ON DELETE CASCADE,
//     previous_mode VARCHAR(20),
//     switched_to VARCHAR(20) CHECK (switched_to IN ('sender', 'traveler')),
//     context TEXT,
//     switched_at TIMESTAMPTZ DEFAULT NOW()
// );

pub struct CreateLogs {
    previous_mode: String,
    switched_to: String,
    context: String,
    switched_at: String,
}

pub async fn create_mode_switch_logs(logs: web::Json<CreateLogs>, pool: web::Data<PgPool>,) -> HttpResponse {
    let result = sqlx::query!(
        r#"INSERT INTO logs (
        id,
        user_id,
        previous_mode,
        switched_to,
        context,
        switched_at
    ) VALUES ($1, $2, $3, $4, $5, $6)"#,
     Uuid::new_v4(),
     logs.user_id,
     logs.previous_mode,
     logs.switched_to,
     logs.context,
    Utc::now()  
    ).execute(pool.get_ref()).await;

    match result {
        oK(_) => HttpResponse::Created().finish(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
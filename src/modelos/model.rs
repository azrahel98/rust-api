use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct Usuario {
    pub id: i32,
    pub nickname: String,
    pub nombre: String,
    pub password: String,
    pub created_at: sqlx::types::chrono::DateTime<Local>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TrabajadoresVw {
    pub dni: String,
    pub nombre: String,
    pub contratos: i64,
    pub activos: Option<i32>,
}

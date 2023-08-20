use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct DocsDate {
    pub dni: String,
    pub doc: i32,
    pub id: i32,
    pub fecha: Option<sqlx::types::chrono::NaiveDate>,
    pub asunto: Option<String>,
    pub descripcion: Option<String>,
    pub referencia: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct DocsRange {
    pub dni: String,
    pub doc: i32,
    pub id: i32,
    pub inicio: Option<sqlx::types::chrono::NaiveDate>,
    pub fin: Option<sqlx::types::chrono::NaiveDate>,
    pub asunto: Option<String>,
    pub descripcion: Option<String>,
    pub referencia: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct Registros {
    pub dni: String,
    pub doc: i32,
    pub id: i32,
    pub inicio: Option<sqlx::types::chrono::NaiveDate>,
    pub fin: Option<sqlx::types::chrono::NaiveDate>,
    pub asunto: Option<String>,
    pub descripcion: Option<String>,
    pub referencia: Option<String>,
}

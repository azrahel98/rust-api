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
pub struct Reloj {
    pub dni: String,
    pub entrada: Option<sqlx::types::chrono::NaiveTime>,
    pub entrada2: Option<sqlx::types::chrono::NaiveTime>,
    pub salida: Option<sqlx::types::chrono::NaiveTime>,
    pub tardanza: Option<sqlx::types::chrono::NaiveTime>,
    pub fecha: Option<sqlx::types::chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct RegistrosReloj {
    pub dni: Option<String>,
    pub fecha: Option<sqlx::types::chrono::NaiveDate>,
    pub tardanza: Option<f32>,
    pub falta: Option<f32>,
}

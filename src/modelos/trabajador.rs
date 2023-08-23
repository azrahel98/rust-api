use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// tabla
pub struct TrabajadorInfo {
    pub dni: String,
    pub nombre: String,
    pub sexo: sqlx::types::JsonValue,
    pub nacimiento: Option<sqlx::types::chrono::NaiveDate>,
    pub direccion: Option<String>,
    pub telf: Option<String>,
    pub email: Option<String>,
    pub discapacitado: sqlx::types::JsonValue,
    pub fotosheck: sqlx::types::JsonValue,
    pub cussp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct ContratosInfo {
    pub dni: String,
    pub activo: sqlx::types::JsonValue,
    pub numero: Option<String>,
    pub sueldo: Option<i64>,
    pub ingreso: Option<sqlx::types::chrono::NaiveDate>,
    pub convocatoria: Option<i32>,
    pub convocatoria_s: Option<String>,
    pub renuncia: Option<sqlx::types::chrono::NaiveDate>,
    pub area: String,
    pub cargo: String,
    pub regimen: sqlx::types::JsonValue,
    pub f1: Option<String>,
    pub f2: Option<String>,
    pub f3: Option<String>,
    pub f4: Option<String>,
    pub f5: Option<String>,
    pub f6: Option<String>,
    pub f7: Option<String>,
    pub f8: Option<String>,
    pub f9: Option<String>,
    pub f10: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// view
pub struct TrabajadorBasic {
    pub dni: String,
    pub sexo: Option<String>,
    pub nombre: String,
    pub area: String,
    pub cargo: String,
}

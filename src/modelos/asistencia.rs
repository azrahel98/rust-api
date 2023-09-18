use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAsistenciaRegistro {
    pub dni: String,
    pub mes: i32,
    pub year: i32,
    pub registros: Option<Vec<Registros>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Registros {
    pub tardanza: Option<i32>,
    pub fecha: NaiveDate,
    pub falta: bool,
}

use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::{
    middleware::{self, sa::ResponseBody},
    modelos::docs::RegistrosReloj,
    AppState,
};

#[post("/", wrap = "middleware::sa::JWT")]
pub async fn buscar_asistencia(
    data: web::Data<AppState>,
    body: web::Json<Value>,
) -> impl Responder {
    if body.get("dni").is_none() || body.get("mes").is_none() || body.get("year").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("dni").unwrap().is_string()
        || !body.get("mes").unwrap().is_i64()
        || !body.get("year").unwrap().is_i64()
    {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let asistencia = sqlx::query_as!(
        RegistrosReloj,
        r#"SELECT
            dni,
            fecha,
            CONVERT( AES_DECRYPT( tardanza, ? ),SIGNED) tardanza,
            CONVERT( AES_DECRYPT( falta, ? ),SIGNED) falta 
        FROM
            asistencia
        WHERE
            dni = ? and year(fecha) = ? and month(fecha) = ? 
        order by fecha desc
        "#,
        body.get("dni").unwrap().as_str(),
        body.get("mes").unwrap(),
        body.get("year").unwrap()
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": asistencia.len(),
        "asistencia": asistencia
    });

    Ok(HttpResponse::Ok().json(json_response))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/asistencia").service(buscar_asistencia);

    conf.service(scope);
}

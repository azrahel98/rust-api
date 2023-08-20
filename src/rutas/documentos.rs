use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::{
    middleware::{self, sa::ResponseBody},
    modelos::docs::{DocsDate, DocsRange},
    AppState,
};

#[post("/", wrap = "middleware::sa::JWT")]
pub async fn buscar_trabajadores(
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

    let docs = sqlx::query_as!(
        DocsDate,
        r#"SELECT
                dni,
                doc as doc,
                id ,
                fecha,
                CAST(AES_DECRYPT(asunto,?) as CHAR) asunto,
                CAST(AES_DECRYPT(descripcion,?) as CHAR) descripcion,
                CAST(AES_DECRYPT(referencia,?) as CHAR) referencia
            FROM
                detalledoc 
            WHERE
                dni = ? 
                AND MONTH ( fecha ) = ? 
                AND YEAR ( fecha ) = ? 
                AND active = 'Y'
        "#,
        std::env::var("AES").unwrap(),
        std::env::var("AES").unwrap(),
        std::env::var("AES").unwrap(),
        body.get("dni").unwrap().as_str(),
        body.get("mes").unwrap(),
        body.get("year").unwrap()
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let ranges = sqlx::query_as!(
        DocsRange,
        r#"SELECT
            dni,
            doc,
            id,
            inicio,
            fin,
            CAST( AES_DECRYPT( asunto,?) AS CHAR ) asunto,
            CAST( AES_DECRYPT( descripcion,?) AS CHAR ) descripcion,
            CAST( AES_DECRYPT( referencia,?) AS CHAR ) referencia 
        FROM
            detalledoc 
        WHERE
            fin >= ? 
            AND YEAR ( fin ) = ? 
            AND dni = ? 
            AND active = 'Y'
        "#,
        std::env::var("AES").unwrap(),
        std::env::var("AES").unwrap(),
        std::env::var("AES").unwrap(),
        format!(
            "{}-{}-01",
            body.get("year").unwrap(),
            body.get("mes").unwrap()
        ),
        body.get("year").unwrap(),
        body.get("dni").unwrap().as_str(),
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": ranges.len(),
        "documentos": {
            "registros":[],
            "doc":docs,
            "ranges":ranges
        }
    });

    Ok(HttpResponse::Ok().json(json_response))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/doc").service(buscar_trabajadores);

    conf.service(scope);
}

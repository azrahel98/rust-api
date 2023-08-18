use actix_web::{post, web, HttpResponse, Responder};
use serde_json::Value;

use crate::{
    middleware::{self, sa::ResponseBody},
    modelos, AppState,
};

use modelos::model::TrabajadoresVw;

#[post("/search", wrap = "middleware::sa::JWT")]
pub async fn note_list_handler(
    data: web::Data<AppState>,
    body: web::Json<Value>,
) -> impl Responder {
    if body.get("nombre").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("nombre").unwrap().is_string() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let notes: Vec<TrabajadoresVw> = sqlx::query_as!(
        TrabajadoresVw,
        r#"SELECT * FROM trabajadores_cactivos_vw WHERE nombre like ?"#,
        "%".to_owned() + body.get("nombre").unwrap().as_str().unwrap() + "%"
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });

    Ok(HttpResponse::Ok().json(json_response))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/employ").service(note_list_handler);

    conf.service(scope);
}

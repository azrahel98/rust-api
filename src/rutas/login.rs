use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::Value;

use crate::{
    middleware::{self, sa::ResponseBody},
    modelos, AppState,
};

use modelos::model::Usuario;

#[get("/", wrap = "middleware::sa::JWT")]
pub async fn note_list_handler(data: web::Data<AppState>) -> impl Responder {
    let notes: Vec<Usuario> = sqlx::query_as!(Usuario, "select * from usuario")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });

    HttpResponse::Ok().json(json_response)
}

#[post("/login")]
pub async fn login(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("nickname").is_none() || body.get("password").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string()
            }
        )));
    }

    let usuario = sqlx::query_as!(
        Usuario,
        "select * from usuario where nickname = ?",
        body.get("nickname").unwrap().as_str()
    )
    .fetch_one(&data.db)
    .await;

    match usuario {
        Ok(us) => {
            if us.password != body["password"].as_str().unwrap() {
                return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                    ResponseBody {
                        message: "contraseña incorrecta".to_string()
                    }
                )));
            }

            let token = middleware::jwt::generate_token(us.id, 3);

            let json_response = serde_json::json!({
                "status": "success",
                "token": token
            });

            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(_) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "usuario no encontrado".to_string()
                }
            )));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(note_list_handler).service(login);

    conf.service(scope);
}

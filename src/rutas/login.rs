use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos, AppState,
};

use modelos::model::Usuario;

#[get("/", wrap = "middleware::sa::JWT")]
pub async fn note_list_handler(data: web::Data<AppState>) -> impl Responder {
    let notes: Vec<Usuario> = sqlx::query_as!(Usuario, "select id,CAST(AES_DECRYPT(nickname,?) as CHAR) nickname,CAST(AES_DECRYPT(password,?) AS CHAR) password,CAST(AES_DECRYPT(nombre,?) AS CHAR) nombre,lvl, created_at from usuario", KEY,
  KEY,
    KEY)
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
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    println!("{}", KEY);

    let usuario = sqlx::query_as!(
        Usuario,
        "select id,CAST(AES_DECRYPT(nickname,?) as CHAR) nickname,CAST(AES_DECRYPT(password,?) AS CHAR) password,CAST(AES_DECRYPT(nombre,?) AS CHAR) nombre,lvl, created_at from usuario where CAST(AES_DECRYPT(nickname,?) as CHAR) = ?",
        KEY,
        KEY,
        KEY,
        KEY,
        body.get("nickname").unwrap().as_str()
    )
    .fetch_one(&data.db)
    .await;

    match usuario {
        Ok(us) => {
            if us.password != Some(body["password"].as_str().unwrap().to_string()) {
                return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                    ResponseBody {
                        message: "contraseña incorrecta".to_string(),
                        code: Some("2".to_string())
                    }
                )));
            }

            let token = middleware::jwt::generate_token(us.id, us.lvl.unwrap(), us.nombre.unwrap());

            let json_response = serde_json::json!({
                "status": "success",
                "token": token
            });

            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(_) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "usuario no encontrado".to_string(),
                    code: Some("1".to_string())
                }
            )));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(note_list_handler).service(login);

    conf.service(scope);
}

use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos::{
        model::TrabajadoresVw,
        trabajador::{ContratosInfo, TrabajadorBasic, TrabajadorInfo},
    },
    AppState,
};

#[post("/search", wrap = "middleware::sa::JWT")]
pub async fn buscar_trabajadores(
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
        "results": notes.len(),
        "trabajadores": notes
    });

    Ok(HttpResponse::Ok().json(json_response))
}

#[get("/{dni}", wrap = "middleware::sa::JWT")]
async fn buscar_by_dni(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    println!("{}", path.as_str().chars().all(|c| c.is_digit(10)));

    if path.as_str().len() != 8 || !path.as_str().chars().all(|c| c.is_digit(10)) {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "formato de dni incorrecto".to_string(),
                code: Some("1".to_string())
            }
        )));
    }

    let employ = sqlx::query_as!(
        TrabajadorInfo,
        "select dni,nombre,sexo,nacimiento,discapacitado,fotosheck,cussp,	CAST(aes_decrypt( direccion, ? ) AS CHAR) direccion,CAST(aes_decrypt( telf, ? ) AS CHAR) telf,CAST(aes_decrypt( email, ? ) AS CHAR) email  from datos_generales where dni = ?",
        KEY,
        KEY,
        KEY,
        path.as_str()
    )
    .fetch_one(&data.db)
    .await;

    match employ {
        Ok(e) => {
            let json_response = serde_json::json!({
                "result": e,

            });

            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(_) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "trabajador no existe".to_string(),
                    code: Some("1".to_string())
                }
            )));
        }
    }
}

#[get("/contrato/{dni}", wrap = "middleware::sa::JWT")]
async fn contratos_by_dni(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    println!("{}", path.as_str().chars().all(|c| c.is_digit(10)));

    if path.as_str().len() != 8 || !path.as_str().chars().all(|c| c.is_digit(10)) {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "formato de dni incorrecto".to_string(),
                code: Some("1".to_string())
            }
        )));
    }

    let notes: Vec<ContratosInfo> = sqlx::query_as!(
        ContratosInfo,
        "select * from contratos_vw where dni =?",
        path.as_str(),
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": notes.len(),
        "contratos": notes
    });

    Ok(HttpResponse::Ok().json(json_response))
}

#[get("/info/{dni}", wrap = "middleware::sa::JWT")]
async fn buscar_by_dni_basico(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    if path.as_str().len() != 8 || !path.as_str().chars().all(|c| c.is_digit(10)) {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "formato de dni incorrecto".to_string(),
                code: Some("1".to_string())
            }
        )));
    }

    let employ = sqlx::query_as!(
        TrabajadorBasic,
        "select * from informacion_basica_vw where dni = ?",
        path.as_str()
    )
    .fetch_one(&data.db)
    .await;

    match employ {
        Ok(e) => {
            let json_response = serde_json::json!({
                "result": e,

            });

            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(_) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "trabajador no existe".to_string(),
                    code: Some("1".to_string())
                }
            )));
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/employ")
        .service(buscar_trabajadores)
        .service(contratos_by_dni)
        .service(buscar_by_dni_basico)
        .service(buscar_by_dni);

    conf.service(scope);
}

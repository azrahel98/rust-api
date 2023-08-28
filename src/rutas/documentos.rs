use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos::docs::{DocId, DocSql, DocsDate, DocsRange, Reloj},
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
        KEY,
        KEY,
        KEY,
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
        KEY,
        KEY,
        KEY,
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

    let asistencia = sqlx::query_as!(
        Reloj,
        "select dni,CAST(entrada as time) entrada,CAST(entrada2 as time) entrada2,CAST(salida as time) salida,CAST(tardanza as time) tardanza,fecha from registros_hora WHERE dni = ? and MONTH(fecha) = ? and year(fecha) = ?",
        body.get("dni").unwrap().as_str(),
        body.get("mes").unwrap(),
        body.get("year").unwrap(),
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": ranges.len(),
        "documentos": {
            "registros":asistencia,
            "doc":docs,
            "ranges":ranges
        }
    });

    Ok(HttpResponse::Ok().json(json_response))
}

#[post("/addoc", wrap = "middleware::sa::JWT")]
pub async fn add_doc(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("nombre").is_none() || body.get("fecha").is_none() || body.get("tipo").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("nombre").unwrap().is_string()
        || !body.get("fecha").unwrap().is_string()
        || !body.get("tipo").unwrap().is_number()
    {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if let Err(_) =
        NaiveDate::parse_from_str(body.get("fecha").unwrap().as_str().unwrap(), "%Y-%m-%d")
    {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "Formato de Fecha incorrecto".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let query_insert =
        sqlx::query(r#"insert into documentos(fecha,nombre,tipo) values(?,AES_ENCRYPT(?,?),?)"#)
            .bind(body.get("fecha").unwrap().as_str().unwrap())
            .bind(body.get("nombre").unwrap().as_str().unwrap())
            .bind(KEY)
            .bind(body.get("tipo").unwrap().as_i64().unwrap())
            .execute(&data.db)
            .await;

    match query_insert {
        Ok(ec) => Ok(HttpResponse::Ok().json(ec.last_insert_id())),
        Err(e) => {
            println!("{}", e);
            match e.into_database_error() {
                Some(cod) => {
                    if cod.is_foreign_key_violation() {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: "documento id no existe".to_string(),
                                code: Some("1".to_string())
                            }
                        )));
                    } else if cod.is_check_violation() {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: "validacion de datos incorrecta".to_string(),
                                code: Some("2".to_string())
                            }
                        )));
                    } else {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: cod.to_string(),
                                code: Some("3".to_string())
                            }
                        )));
                    }
                }
                None => {
                    let qhe = sqlx::query_as!(
                        DocId,
                        "select docid from documentos where aes_decrypt(nombre,?) = ? ",
                        KEY,
                        body.get("nombre").unwrap().as_str().unwrap()
                    )
                    .fetch_one(&data.db)
                    .await;

                    match qhe {
                        Ok(doc) => Ok(HttpResponse::Ok().json(doc.docid)),
                        Err(_) => {
                            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                                ResponseBody {
                                    message: "error desconocido".to_string(),
                                    code: Some("3".to_string())
                                }
                            )))
                        }
                    }
                }
            }
        }
    }
    // Ok(HttpResponse::Ok().json(2))
}

#[post("/addetalle", wrap = "middleware::sa::JWT")]
pub async fn add_detalle(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("dni").is_none()
        || body.get("doc").is_none()
        || body.get("fecha").is_none()
        || body.get("descripcion").is_none()
    {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("doc").unwrap().is_i64() || !body.get("asunto").unwrap().is_string() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let query = format!(
        "INSERT INTO detalledoc (doc,dni,fecha,asunto,referencia,descripcion,inicio,fin) VALUES
    (
        {},
        '{}',
        {},
        AES_ENCRYPT('{}','{}'),
        AES_ENCRYPT({},'{}'),
        AES_ENCRYPT('{}','{}'),
        {},
        {}
    )",
        body.get("doc").unwrap().as_i64().unwrap(),
        body.get("dni").unwrap().as_str().unwrap(),
        body.get("fecha").unwrap_or(&Value::Null),
        body.get("asunto").unwrap().as_str().unwrap(),
        KEY,
        body.get("referencia").unwrap_or(&Value::Null),
        KEY,
        body.get("descripcion").unwrap().as_str().unwrap(),
        KEY,
        body.get("inicio").unwrap_or(&Value::Null),
        body.get("fin").unwrap_or(&Value::Null)
    );

    println!("{}", query);

    let query_insert = sqlx::query(&query).execute(&data.db).await;

    match query_insert {
        Ok(e) => Ok(HttpResponse::Ok().json(ResponseBody {
            message: e.last_insert_id().to_string(),
            code: Some(1.to_string()),
        })),
        Err(e) => {
            println!("{}", e);
            match e.into_database_error() {
                Some(cod) => {
                    if cod.is_foreign_key_violation() {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: "documento id no existe".to_string(),
                                code: Some("1".to_string())
                            }
                        )));
                    } else if cod.is_check_violation() {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: "validacion de datos incorrecta".to_string(),
                                code: Some("2".to_string())
                            }
                        )));
                    } else {
                        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                            ResponseBody {
                                message: cod.to_string(),
                                code: Some("3".to_string())
                            }
                        )));
                    }
                }
                None => {
                    return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                        ResponseBody {
                            message: "error desconocido".to_string(),
                            code: Some("3".to_string())
                        }
                    )));
                }
            }
        }
    }
}

#[post("/search", wrap = "middleware::sa::JWT")]
pub async fn buscar_doc(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("doc").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("doc").unwrap().is_string() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros no validos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let query = sqlx::query_as!(
        DocSql,
        "select docid,fecha,CAST(AES_DECRYPT(nombre,?) AS CHAR) nombre,tipo from documentos where AES_DECRYPT(nombre,?) = ?",KEY,KEY,body.get("doc").unwrap().as_str())
    .fetch_one(&data.db)
    .await;

    match query {
        Ok(doc) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "documento": doc
        }))),
        Err(_e) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "no hay datos".to_string(),
                    code: Some("2".to_string())
                }
            )))
        }
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/doc")
        .service(buscar_trabajadores)
        .service(add_detalle)
        .service(buscar_doc)
        .service(add_doc);

    conf.service(scope);
}

// let usuario = sqlx::query_as!(
//     Usuario,
//     "select id,CAST(AES_DECRYPT(nickname,?) as CHAR) nickname,CAST(AES_DECRYPT(password,?) AS CHAR) password,CAST(AES_DECRYPT(nombre,?) AS CHAR) nombre, created_at from usuario where CAST(AES_DECRYPT(nickname,?) as CHAR) = ?",
//     KEY,
//     KEY,
//     KEY,
//     KEY,
//     body.get("nickname").unwrap().as_str()
// )
// .fetch_one(&data.db)
// .await;

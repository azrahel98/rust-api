use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use chrono::NaiveDate;
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos::docs::{DocId, DocSql, DocSs, Docs, DocsRange, Reloj},
    AppState,
};

#[post("/", wrap = "middleware::sa::JWT")]
pub async fn buscar_docs(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
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

    let ranges = sqlx::query_as!(
        Docs,
        r#"SELECT
        d.dni,
        d.doc AS doc,
        d.id ,
        CAST( AES_DECRYPT( dc.nombre, ? ) AS CHAR ) nombre,
        d.fecha,
        CAST( AES_DECRYPT( d.asunto, ? ) AS CHAR ) asunto,
        CAST( AES_DECRYPT( d.descripcion, ? ) AS CHAR ) descripcion,
        CAST( AES_DECRYPT( d.referencia, ? ) AS CHAR ) referencia,
        d.inicio,
        d.fin
    FROM
        detalledoc d
        INNER JOIN documentos dc ON d.doc = dc.docid 
    WHERE
        ( d.dni = ? and d.active = 'Y'
         ) 
        AND (
            ( MONTH ( d.fecha ) = ? AND YEAR ( d.fecha ) = ? ) 
        OR ( d.fin >= ? and MONTH ( d.inicio ) <= ? AND YEAR ( d.fin ) = ?  ) 
        )
        "#,
        KEY,
        KEY,
        KEY,
        KEY,
        body.get("dni").unwrap().as_str(),
        body.get("mes").unwrap().as_i64(),
        body.get("year").unwrap(),
        format!(
            "{}-{}-01",
            body.get("year").unwrap(),
            body.get("mes").unwrap()
        ),
        body.get("mes").unwrap().as_i64(),
        body.get("year").unwrap(),
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let mut documentos: Vec<DocSs> = Vec::new();
    let mut rang: Vec<DocsRange> = Vec::new();

    for x in ranges.iter() {
        if x.fecha.is_none() {
            rang.push(DocsRange {
                dni: x.dni.clone(),
                doc: x.doc,
                nombre: x.nombre.clone(),
                id: x.id,
                inicio: x.inicio,
                fin: x.fin,
                asunto: x.asunto.clone(),
                descripcion: x.descripcion.clone(),
                referencia: x.referencia.clone(),
            })
        } else {
            documentos.push(DocSs {
                dni: x.dni.clone(),
                doc: x.doc,
                nombre: x.nombre.clone(),
                id: x.id,
                fecha: x.fecha,
                asunto: x.asunto.clone(),
                descripcion: x.descripcion.clone(),
                referencia: x.referencia.clone(),
            })
        }
    }

    let asistencia = sqlx::query_as!(
        Reloj,
        "select dni,CAST(entrada as time) entrada,CAST(entrada2 as time) entrada2,CAST(salida as time) salida,fecha from registros_hora WHERE dni = ? and MONTH(fecha) = ? and year(fecha) = ?",
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
            "doc":documentos,
            "ranges":rang
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
        || !body.get("user").unwrap().is_number()
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

    let query_insert = sqlx::query(
        r#"insert into documentos(fecha,nombre,tipo,create_by) values(?,AES_ENCRYPT(?,?),?,?)"#,
    )
    .bind(body.get("fecha").unwrap().as_str().unwrap())
    .bind(body.get("nombre").unwrap().as_str().unwrap())
    .bind(KEY)
    .bind(body.get("tipo").unwrap().as_i64().unwrap())
    .bind(body.get("user").unwrap().as_i64().unwrap())
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

    if !body.get("doc").unwrap().is_i64()
        || !body.get("asunto").unwrap().is_string()
        || !body.get("user").unwrap().is_i64()
    {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let query = format!(
        "INSERT INTO detalledoc (doc,dni,fecha,asunto,referencia,descripcion,inicio,fin,create_by) VALUES
    (
        {},
        '{}',
        {},
        AES_ENCRYPT('{}','{}'),
        AES_ENCRYPT({},'{}'),
        AES_ENCRYPT('{}','{}'),
        {},
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
        body.get("fin").unwrap_or(&Value::Null),
        body.get("user").unwrap().as_i64().unwrap(),
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

#[post("/delete", wrap = "middleware::sa::JWT")]
pub async fn delete_doc(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("id").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("id").unwrap().is_number() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let query_insert = sqlx::query(r#"delete from detalledoc where id = ?"#)
        .bind(body.get("id").unwrap().as_i64().unwrap())
        .execute(&data.db)
        .await;

    match query_insert {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                    ResponseBody {
                        message: "no hay datos".to_string(),
                        code: Some("2".to_string())
                    }
                )));
            } else {
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "deleted"
                })))
            }
        }
        Err(_e) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "error desconocido".to_string(),
                    code: Some("2".to_string())
                }
            )));
        }
    }
    // Ok(HttpResponse::Ok().json(2))
}

#[post("/anulardoc", wrap = "middleware::sa::JWT")]
pub async fn anular_doc(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("id").is_none() && body.get("valor").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    if !body.get("id").unwrap().is_number() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros f".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let activo = if body.get("valor").unwrap().is_boolean() {
        'N'
    } else {
        'Y'
    };

    let querystring = format!(
        "update detalledoc set active = '{}' where id = {}",
        activo,
        body.get("id").unwrap().as_i64().unwrap()
    );

    println!("{}", querystring);

    let query_insert = sqlx::query(&querystring).execute(&data.db).await;

    match query_insert {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                    ResponseBody {
                        message: "no hay datos".to_string(),
                        code: Some("2".to_string())
                    }
                )));
            } else {
                Ok(HttpResponse::Ok().json(serde_json::json!({
                    "status": "UPDATED"
                })))
            }
        }
        Err(_e) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: "error desconocido".to_string(),
                    code: Some("2".to_string())
                }
            )));
        }
    }
    // Ok(HttpResponse::Ok().json(2))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/doc")
        .service(buscar_docs)
        .service(add_detalle)
        .service(buscar_doc)
        .service(delete_doc)
        .service(anular_doc)
        .service(add_doc);

    conf.service(scope);
}

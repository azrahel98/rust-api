use actix_web::{
    post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos::{asistencia::CreateAsistenciaRegistro, docs::RegistrosReloj},
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
            CASE
            WHEN CONVERT(AES_DECRYPT(falta, ?), SIGNED) = 0 THEN FALSE
            ELSE CONVERT(AES_DECRYPT(falta, ?), SIGNED)
          END AS falta
        FROM
            asistencia
        WHERE
            dni = ? and year(fecha) = ? and month(fecha) = ? 
        order by fecha desc
        "#,
        KEY,
        KEY,
        KEY,
        body.get("dni").unwrap().as_str(),
        body.get("year").unwrap(),
        body.get("mes").unwrap()
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

#[post("/agregar", wrap = "middleware::sa::JWT")]
pub async fn agregar_asistencia(
    data: web::Data<AppState>,
    body: web::Json<CreateAsistenciaRegistro>,
) -> impl Responder {
    let query = format!(
        "delete from asistencia where dni = '{}' and month(fecha) = {} and year(fecha) = {}",
        body.dni.as_str(),
        body.mes,
        body.year
    );
    let _query_insert = sqlx::query(&query).execute(&data.db).await.unwrap();

    if let Some(registros) = &body.registros {
        for x in registros {
            let registro_query = sqlx::query(
                r#"insert into asistencia values (?,?,AES_ENCRYPT(?,?),AES_ENCRYPT(?,?))"#,
            )
            .bind(body.dni.as_str())
            .bind(x.fecha)
            .bind(x.tardanza)
            .bind(KEY)
            .bind(x.falta)
            .bind(KEY)
            .execute(&data.db)
            .await;

            if registro_query.is_err() {
                println!("{:?}", registro_query.err());
                return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                    ResponseBody {
                        message: "error en los insert".to_string(),
                        code: Some("3".to_string())
                    }
                )));
            }
        }
    }
    Ok(HttpResponse::Ok().json("Guardado"))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/asistencia")
        .service(agregar_asistencia)
        .service(buscar_asistencia);

    conf.service(scope);
}

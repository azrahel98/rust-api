use actix_web::{
    get, post,
    web::{self},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::{
    middleware::{self, key::KEY, sa::ResponseBody},
    modelos::{
        docs::DocAdenda,
        model::TrabajadoresVw,
        trabajador::{
            CargoAreaSearch, ContratosInfo, ResNuevoContrato, TrabajadorBasic, TrabajadorInfo,
        },
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

#[post("/cargo", wrap = "middleware::sa::JWT")]
pub async fn buscar_cargo(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("nombre").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let resultado: Vec<CargoAreaSearch> = sqlx::query_as!(
        CargoAreaSearch,
        r#"SELECT id,nombre FROM cargo WHERE nombre like ?"#,
        "%".to_owned() + body.get("nombre").unwrap().as_str().unwrap() + "%"
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": resultado.len(),
        "cargos": resultado
    });

    Ok(HttpResponse::Ok().json(json_response))
}

#[post("/area", wrap = "middleware::sa::JWT")]
pub async fn buscar_area(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("nombre").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let resultado: Vec<CargoAreaSearch> = sqlx::query_as!(
        CargoAreaSearch,
        r#"SELECT id,nombre FROM area WHERE nombre like ?"#,
        "%".to_owned() + body.get("nombre").unwrap().as_str().unwrap() + "%"
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "results": resultado.len(),
        "areas": resultado
    });

    Ok(HttpResponse::Ok().json(json_response))
}

#[post("/adenda", wrap = "middleware::sa::JWT")]
pub async fn buscar_adenda(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
    if body.get("dni").is_none() {
        return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
            ResponseBody {
                message: "parametros incorrectos".to_string(),
                code: Some("3".to_string())
            }
        )));
    }

    let resultado = sqlx::query_as!(
        DocAdenda,
        "select
        c.dni,
        dg.nombre nombre,
        CAST(aes_decrypt(dg.direccion, ?) AS CHAR) direccion,
        c.numero,
        c.ingreso,
        ar.nombre area,
        cr.nombre cargo,
        rg.nombre regimen,
        dg.ruc
      from
        contrato c
        inner join datos_generales dg on c.dni = dg.dni
        inner join area ar on c.area = ar.id
        inner join cargo cr on c.cargo = cr.id
        inner join regimen rg on c.regimen = rg.id
      where
        c.dni = ?
      limit
        1",
        KEY,
        body.get("dni").unwrap().as_str().unwrap()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();

    let json_response = serde_json::json!({
        "resultado":resultado
    });

    Ok(HttpResponse::Ok().json(json_response))
}

/// .
#[post("/addcontrato", wrap = "middleware::sa::JWT")]
pub async fn add_contrato(
    data: web::Data<AppState>,
    body: web::Json<ResNuevoContrato>,
) -> impl Responder {
    let mut funcion_id = 1;

    if body.funciones.is_some() {
        let mut funcione: Vec<String> = Vec::new();
        let mut valores: Vec<String> = Vec::new();
        for x in 0..body.funciones.clone().unwrap().len() {
            funcione.push(format!("f{}", x + 1))
        }
        for x in body.funciones.clone().unwrap().iter() {
            valores.push(format!("'{}'", x))
        }
        let query = format!(
            "insert into funciones_contratos({}) values ({})",
            funcione.join(","),
            valores.join(",")
        );
        let query_insert = sqlx::query(&query).execute(&data.db).await.unwrap();
        funcion_id = query_insert.last_insert_id();
    }
    let query = format!(
        "INSERT INTO contrato (dni,numero,ingreso,convocatoria_s,area,cargo,regimen,funcion,sueldo) VALUES
    (
        '{}',{},'{}','{}',{},{},{},{},{}
    )",
    body.dni,
    body.contrato,
    body.ingreso,
    body.convocatoria.clone().unwrap_or("".to_string()),
    body.area,
    body.cargo,
    body.regimen,
    funcion_id,
    body.sueldo
    );

    let query_insert = sqlx::query(&query).execute(&data.db).await;

    match query_insert {
        Ok(e) => Ok(HttpResponse::Ok().json(e.last_insert_id())),
        Err(e) => {
            return Err(actix_web::error::ErrorUnauthorized(serde_json::json!(
                ResponseBody {
                    message: e.to_string(),
                    code: Some("3".to_string())
                }
            )));
        }
    }
}
#[post("/addrenuncia", wrap = "middleware::sa::JWT")]
pub async fn agregar_renuncia(data: web::Data<AppState>, body: web::Json<Value>) -> impl Responder {
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

    let query_insert =
        sqlx::query(r#"update contrato set renuncia  = ?, activo = 'N' where id = ?"#)
            .bind(body.get("renuncia").unwrap().as_str().unwrap())
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

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/employ")
        .service(buscar_trabajadores)
        .service(contratos_by_dni)
        .service(buscar_by_dni_basico)
        .service(buscar_area)
        .service(add_contrato)
        .service(agregar_renuncia)
        .service(buscar_cargo)
        .service(buscar_adenda)
        .service(buscar_by_dni);
    conf.service(scope);
}

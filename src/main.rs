use actix_cors::Cors;
use actix_service::Service;
use actix_web::middleware::Logger;
use actix_web::HttpMessage;
use actix_web::{web, App, HttpServer};
use chrono::FixedOffset;
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

mod middleware;
mod modelos;
mod rutas;

pub struct AppState {
    pub db: sqlx::Pool<sqlx::MySql>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("tz", "America/Lima");
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        // .connect_with(opts)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(rutas::login::config)
            .configure(rutas::empleados::config)
            .configure(rutas::documentos::config)
            .configure(rutas::asistencia::config)
            .wrap_fn(|re, rec| {
                re.extensions_mut()
                    .insert(FixedOffset::east_opt(-5 * 3600).unwrap());
                rec.call(re)
            })
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

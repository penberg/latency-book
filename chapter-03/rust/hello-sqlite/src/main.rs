use actix_web::{
    error::ErrorInternalServerError, web, App, Error,
    HttpServer,
};
use rusqlite::Connection;
use std::env;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let conn = Arc::new(Mutex::new(Connection::open(
        database_url,
    )?));
    let app = move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .service(
                web::resource("/hello")
                    .route(web::get().to(say_hello)),
            )
    };
    Ok(HttpServer::new(app)
        .bind("127.0.0.1:8080")?
        .run()
        .await?)
}

async fn say_hello(
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> anyhow::Result<String, Error> {
    let conn = conn.lock().unwrap();
    let result = conn
        .query_row("SELECT 'hello world'", [], |row| {
            row.get(0)
        })
        .map_err(ErrorInternalServerError)?;
    Ok(result)
}

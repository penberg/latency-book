use actix_web::{
    error::ErrorInternalServerError, web, App, Error,
    HttpServer,
};
use libsql_client::{
    client::GenericClient, new_client_from_config, Config,
    DatabaseClient,
};
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL")?;
    let database_token = env::var("DATABASE_TOKEN")?;
    let client = new_client_from_config(Config {
        url: Url::parse(&database_url)?,
        auth_token: Some(database_token),
    })
    .await?;
    let client = Arc::new(Mutex::new(client));
    let app = move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
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
    conn: web::Data<Arc<Mutex<GenericClient>>>,
) -> anyhow::Result<String, Error> {
    let conn = conn.lock().await;
    let rs = conn
        .execute("SELECT 'hello world'")
        .await
        .map_err(ErrorInternalServerError)?;
    let row = rs.rows.first().ok_or_else(|| {
        ErrorInternalServerError("no rows found")
    })?;
    Ok(row.values[0].to_string())
}

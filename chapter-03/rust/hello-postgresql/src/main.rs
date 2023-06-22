use actix_web::{
    error::ErrorInternalServerError, web, App, Error,
    HttpServer,
};
use mobc::Pool;
use mobc_postgres::PgConnectionManager;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::{env, str::FromStr};
use tokio_postgres::Config;

type DatabasePool =
    Pool<PgConnectionManager<MakeTlsConnector>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_pool()?;
    let app = move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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

fn create_pool() -> anyhow::Result<DatabasePool> {
    let database_url = env::var("DATABASE_URL")?;
    let config = Config::from_str(&database_url)?;
    let builder = SslConnector::builder(SslMethod::tls())?;
    let tls = MakeTlsConnector::new(builder.build());
    let manager = PgConnectionManager::new(config, tls);
    let pool = Pool::builder().max_open(20).build(manager);
    Ok(pool)
}

async fn say_hello(
    pool: web::Data<DatabasePool>,
) -> anyhow::Result<String, Error> {
    let conn = pool
        .get()
        .await
        .map_err(ErrorInternalServerError)?;
    let result = conn
        .query_one("SELECT 'hello world'", &[])
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(result.get(0))
}

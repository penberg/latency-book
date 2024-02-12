use actix_web::{
    error::ErrorInternalServerError, web::{self, Data}, App, Error, HttpServer,
};
use mobc::Pool;
use mobc_postgres::PgConnectionManager;
use moka::sync::Cache;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::{env, str::FromStr, sync::Arc, time::Duration};
use tokio_postgres::Config;

type DatabasePool =
    Pool<PgConnectionManager<MakeTlsConnector>>;

struct AppState {
    pool: DatabasePool,
    cache: Cache<String, String>,    
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = create_pool()?;
    let cache = create_cache();
    let data = Arc::new(AppState { pool, cache });
    let app = move || {
        App::new()
            .app_data(Data::new(data.clone()))
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

fn create_cache() -> Cache<String, String> {
    Cache::builder()
        .time_to_live(Duration::from_secs(5))
        .build()
}

const CACHE_KEY: &str = "RESULT_KEY";

async fn say_hello(
    data: web::Data<Arc<AppState>>,
) -> anyhow::Result<String, Error> {
    if let Some(result) = data.cache.get(CACHE_KEY) {
        Ok(result)
    } else {
        let result = fetch_value(&data.pool).await?;
        data.cache.insert(CACHE_KEY.to_owned(), result.to_owned());
        Ok(result)
    }
}

async fn fetch_value(pool: &DatabasePool)
    -> anyhow::Result<String, Error> {
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

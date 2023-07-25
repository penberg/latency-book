use libsql_client::{workers::*, *};
use worker::*;

#[event(fetch)]
pub async fn main(
    req: Request,
    env: Env,
    _ctx: worker::Context,
) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/hello", |_, ctx| async move {
            let db = Client::from_ctx(&ctx).await;
            let db = match db {
                Ok(db) => db,
                Err(e) => {
                    return Response::error(
                        format!(
                        "Error connecting to database: {e}"
                    ),
                        500,
                    )
                }
            };
            let rs = db
                .execute("SELECT 'hello world'")
                .await
                .unwrap();
            let row = match rs.rows.first() {
                Some(row) => row,
                None => {
                    return Response::error(
                        "no rows found",
                        500,
                    )
                }
            };
            Response::ok(row.values[0].to_string())
        })
        .run(req, env)
        .await
}

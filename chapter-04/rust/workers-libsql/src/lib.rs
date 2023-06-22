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
            Response::ok("hello, world".to_string())
        })
        .run(req, env)
        .await
}

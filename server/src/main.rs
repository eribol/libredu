extern crate tide;
extern crate async_std;
extern crate serde;
extern crate dotenv;
//extern crate env_logger;
extern crate uuid;
//extern crate log;
use async_std::task;
use tide::Request;
use tide::StatusCode;
use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;

mod model;
mod views;
mod request;
mod api;
mod routes;
mod middlewares;

/*const DOMAIN: &str = match &env::var("DOMAIN_NAME"){
    Ok(n) => n,
    Err(_) => "None".to_string()
};*/

#[derive(Debug, Clone)]
pub struct AppState{
    pub db_pool: PgPool
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    tide::log::start();
    task::block_on(async {
        let pool = PgPool::new(&env::var("DATABASE_URL")?).await?;
        let state = AppState{
            db_pool: pool.clone()
        };
        let mut app = crate::routes::routes(state.clone());
        app.at("/static").serve_dir("./client/pkg/")?;
        app.at("/sse").get(tide::sse::endpoint(sse));
        app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}

async fn sse(_req: tide::Request<AppState>, sender: tide::sse::Sender) -> tide::Result<()>{
    let _res = tide::Response::new(StatusCode::Ok);
    sender.send("post", "3", None).await?;
    Ok(())
}
async fn index(_req: Request<AppState>)->tide::Result{
    use http_types::Body;
    let mut res = tide::Response::new(StatusCode::Ok);
    res.set_body(Body::from_file("./server/templates/index.html").await?);
    res.insert_header("content-type", "text/html");
    Ok(res)
}

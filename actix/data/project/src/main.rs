use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::web::{Either, Json, Form};
use mysql::{prelude::*, Pool, Opts};

use env_logger;
use log;
use dotenv;

#[derive(Deserialize)]
struct Register {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: String,
    pub password: String,
}

async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register { username, password } = form.into_inner();
    let db: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let url = Opts::from_url(&db).unwrap();
    let pool = Pool::new(url).unwrap();
    let mut conn: mysql::PooledConn = pool.get_conn().unwrap();
    let res = conn.exec_drop(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        (username.clone(), password.clone()),
    );
    match res {
        Ok(_) => println!("✅ Insert to the table is successful!"),
        Err(err) => println!("🔥 Failed to insert to the table: {:?}", err),
    };
    log::info!("username: {username}, password: {password}");
    format!("You are {username}, and password is {password}.")
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/register", web::post().to(register))
    })
    .bind(("172.111.0.3", 80))?
    .run()
    .await
}
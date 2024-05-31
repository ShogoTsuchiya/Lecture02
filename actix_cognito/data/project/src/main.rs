use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use actix_web::web::{Either, Json, Form};
use aws_credential_types::Credentials;
use aws_types::region::Region;
use aws_sdk_cognitoidentityprovider::error::{SignUpError, SignUpErrorKind};
use aws_sdk_cognitoidentityprovider::{Client, model::AttributeType, Config};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use env_logger;
use log;
use dotenv;
#[derive(Deserialize)]
struct Register {
    username: String,
    email: String,
    password: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn generate_secret_hash(name: &str) -> String {
    let client_id: String = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let app_secret: String = std::env::var("APP_CLIENT_SECRET").expect("APP_CLIENT_SECRET must be set");
    let message = name.to_string() + &client_id;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&app_secret.as_bytes()).expect("HAMC can take key of any size");
    mac.update(message.as_bytes());
    let secret_hash = mac.finalize().into_bytes();
    general_purpose::STANDARD.encode(secret_hash).to_string()
}

pub async fn cognito_client() -> Result<Client> {
    let creds = Credentials::from_keys(
        "AKIARHXN4ZT3WT77I6QI",
        "CNlhF7ASHy/kx9EsTjnQCZAnOXq2ZGWPcF+0C2vl",
        None,
    );
    let config = Config::builder()
        .region(Region::new("ap-northeast-1"))
        .credentials_provider(creds)
        .build();

    let client = Client::from_conf(config);
    Ok(client)
}

async fn register(form: Either<Json<Register>, Form<Register>>) -> impl Responder {
    let Register { username, email, password } = form.into_inner();
    let mut attributes: Vec<AttributeType> = Vec::new();
    let attribute = AttributeType::builder().set_name(Some("name".to_string())).set_value(Some(username.clone())).build();
    attributes.push(attribute);
    let attribute = AttributeType::builder().set_name(Some("email".to_string())).set_value(Some(email.clone())).build();
    attributes.push(attribute);
    let client_id: String = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client: Client = cognito_client().await.unwrap();
    let secret_hash: String = generate_secret_hash(&email.clone());
    match client.sign_up()
        .set_username(Some(email.clone()))
        .set_password(Some(password.clone()))
        .set_client_id(Some(client_id))
        .set_secret_hash(Some(secret_hash))
        .set_user_attributes(Some(attributes))
        .send().await {
            Ok(_) => {
                println!("Succeed.");
            },
            Err(_e) => match _e.into_service_error() {
                println!("Error:{:?}", _e);
            },
        };
    log::info!("username: {username}, password: {password}");
    format!("Hi, {username}. Sent email to {email}.")
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
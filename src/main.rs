pub mod schema;

use std::env;

use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::{sql_query, RunQueryDsl};
use todo_actix::config::Config;
use todo_actix::handlers::app_config;
use todo_actix::models::user::NewUser;
use tracing::{info, instrument};


async fn test() -> impl Responder {
    // let token = AccessToken::new().generate_token("quangnam11032001@gmail.com".to_owned() , 24);
    // let a = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzUxNjIwMjksImVtYWlsIjoicXVhbmduYW0xMTAzMjAwMUBnbWFpbC5jb20ifQ.bn-oeBynFRjWkR9Y04QjA1hHpmDkECQXhpA0mSrsrvg");
  
    // let token = AccessToken::new().validate_token(a).unwrap_or("HUHU".to_owned());
    // HttpResponse::Ok().json(token)
    HttpResponse::Ok().json("fu")
}

async fn register(user: web::Form<NewUser>) -> impl Responder {
    // let token = AccessToken::new().generate_token("quangnam11032001@gmail.com".to_owned() , 24);
    // let a = String::from("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE2NzUxNjIwMjksImVtYWlsIjoicXVhbmduYW0xMTAzMjAwMUBnbWFpbC5jb20ifQ.bn-oeBynFRjWkR9Y04QjA1hHpmDkECQXhpA0mSrsrvg");
  
    // let token = AccessToken::new().validate_token(a).unwrap_or("HUHU".to_owned());
    // HttpResponse::Ok().json(token)
    // info!("User: {:?}", user.password);
    // let crypto = Config::from_env().expect("Server configuration").hashing();
    // let hash_password = crypto.hash_password(user.password.clone()).await.unwrap();
    HttpResponse::Ok().json("hash_password")
}

#[tokio::main]
#[instrument]
async fn main() -> std::io::Result<()> {
   
    let config = Config::from_env().expect("Server configuration");
    let pool = config.db_pool();

    let _ = sql_query("SET TIME ZONE 'Asia/Ho_Chi_Minh'").execute(&mut pool.get().expect("Error connection DB"));


    info!("Starting server at http://{}:{}/", config.host, config.port);
    let hashing = config.hashing();
    
    HttpServer::new(move || {
        App::new()
        .   wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(hashing.clone()))
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}

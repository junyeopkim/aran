use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use aran::merong;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{user_id}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!, {}", friend, user_id, merong()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

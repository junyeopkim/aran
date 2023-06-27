use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;
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
#[derive(Deserialize)]
struct RepoRequest {
    url: String,
    dist: String,
}

async fn app() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
        r#"
        <form action="/app/result" method="post" class="form-example">
        <div class="form-example">
            <label for="repo">Enter repo URL: </label>
            <input type="text" name="url" id="url" required>
        </div>
        <div class="form-example">
            <label for="dist">Enter Dist: </label>
            <input type="text" name="dist" id="dist" required>
        </div>
        <div class="form-example">
            <input type="submit" value="Search!">
        </div>
        </form>
        "#
    )
}

async fn result(info: web::Form<RepoRequest>) -> impl Responder {
    format!("You input repo: {}, dist: {}", info.url, info.dist)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(index)
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(app))
                    .route("/result", web::post().to(result))
            )
    })   
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

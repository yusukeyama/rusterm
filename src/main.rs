use actix_web::{web, get, post, App, HttpResponse, HttpServer, Responder};

#[get("/user/{id}")]
async fn user(web::Path(id): web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("user_id is {}", id))
}

#[post("/")]
async fn post() -> impl Responder {
    HttpResponse::Ok().body("post")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user)
            .service(post)   
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

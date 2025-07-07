use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;


// GET /user/me
async fn me_user() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "User_id": "This is me!"
    }))
}

// GET /user/{id}
async fn user(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "User_id": id
    }))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/user/{id}", web::get().to(user)) 
            .route("/user/me", web::get().to(me_user)) //#error, order matters, fix the error by switching to the first position
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

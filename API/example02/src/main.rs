use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use serde::Deserialize;

// GET /car/{id}
async fn get_car(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "car_id": id
    }))
}

// GET /books/{book_id}
async fn get_book(path: web::Path<i32>) -> impl Responder {
    let book_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "book_id": book_id,
        "title": "Alice´s Adventures in Worderland",
        "author": "Lewis Carroll"
    }))
}

// GET /authors/{author_id}
async fn get_author(path: web::Path<i32>) -> impl Responder {
    let author_id = path.into_inner();
    HttpResponse::Ok().json(json!({
        "author_id": author_id,
        "author": "Lewis Carroll"
    }))
}

// Structure for parsing parameter query
#[derive(Deserialize)]
struct Books2Query {
    year: Option<i32>,
}

// GET /books2?year=...
async fn get_books2(query: web::Query<Books2Query>) -> impl Responder {
    if let Some(year) = query.year {
        HttpResponse::Ok().json(json!({
            "year": year,
            "books": ["Book 1", "Book 2"]
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "books": ["All Books"]
        }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/car/{id}", web::get().to(get_car))
            .route("/books/{book_id}", web::get().to(get_book))
            .route("/authors/{author_id}", web::get().to(get_author))
            .route("/books2", web::get().to(get_books2))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

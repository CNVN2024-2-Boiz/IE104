use actix_web::{App, HttpServer, Result, get, web};

#[get("/{id}")]
async fn greet(path: web::Path<String>) -> Result<String> {
    let id = path.into_inner();
    Ok(format!("The id is: {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}


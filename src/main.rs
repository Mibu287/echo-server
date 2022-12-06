use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/{message}")]
async fn echo(message: actix_web::web::Path<String>) -> impl Responder {
    format!("Echo: {}", message.as_str())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(echo))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

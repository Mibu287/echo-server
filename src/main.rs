use actix_web::{middleware::Logger, App, HttpServer, Responder};

#[actix_web::get("/{message}")]
async fn echo(message: actix_web::web::Path<String>) -> impl Responder {
    format!("Echo: {}", message.as_str())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app = || App::new().service(echo).wrap(Logger::default());
    HttpServer::new(app).bind(("0.0.0.0", 8080))?.run().await
}

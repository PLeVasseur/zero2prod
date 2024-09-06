use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)));
    let bound_server = server.bind("127.0.0.1:8000")?;
    bound_server.run().await
}


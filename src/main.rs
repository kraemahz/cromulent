use actix_web::{web, App, HttpResponse, HttpServer, Responder};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/language_model", web::post().to(language_model_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn language_model_handler(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("Request received: {}", req_body))
}

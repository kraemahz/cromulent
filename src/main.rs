use actix_web::{web, App, HttpResponse, HttpServer, Responder};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/language_model", web::post().to(language_model_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn language_model_handler(req_body: web::Json<LanguageModelRequest>) -> impl Responder {
    let input_text = &req_body.input_text;
    HttpResponse::Ok().body(format!("Parsed text: {}", input_text))
}
#[derive(Deserialize)]
struct LanguageModelRequest {
    input_text: String,
}

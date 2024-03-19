use serde::Deserialize;
use actix_web::Responder;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::env;
async fn language_model_handler(req_body: web::Json<LanguageModelRequest>) -> impl Responder {
    let response =
        forward_to_language_model(LANGUAGE_MODEL_API_ENDPOINT, &req_body.into_inner()).await;

    match response {
        Ok(response_body) => HttpResponse::Ok().json(response_body),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .route("/language_model", web::post().to(language_model_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Deserialize)]
struct LanguageModelRequest {
    // Define the fields expected in the request
}
const LANGUAGE_MODEL_API_ENDPOINT: &str = "http://example.com/api";
async fn forward_to_language_model(endpoint: &str, req_body: &LanguageModelRequest) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Simulate forwarding the request to the language model API
    // This is a placeholder implementation
    Ok(serde_json::Value::String("Success".to_string()))
}
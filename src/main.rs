use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/language_model", web::post().to(language_model_handler)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn language_model_handler(req_body: web::Json<LanguageModelRequest>) -> impl Responder {
    let mut constructed_body = serde_json::json!({ "input_text": req_body.input_text });

    let response = forward_to_language_model(LANGUAGE_MODEL_API_ENDPOINT, &constructed_body).await;

    match response {
        Ok(response_body) => HttpResponse::Ok().json(response_body),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
#[derive(Deserialize)]
struct LanguageModelRequest {
    input_text: String,
}
async fn forward_to_language_model(api_endpoint: &str, input_data: &Value) -> Result<(), Error> {
    let client = Client::new();
    let res = client.post(api_endpoint).json(input_data).send().await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    // Optionally, handle the response body
    // let response_body = res.text().await?;

    Ok(())
}
const LANGUAGE_MODEL_API_ENDPOINT: &str = "http://example.com/api/language_model";
#[derive(Deserialize, Serialize)]
struct LanguageModelResponse {
    generated_text: String,
}

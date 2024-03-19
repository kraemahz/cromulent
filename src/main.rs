use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
async fn language_model_handler(req_body: web::Json<LanguageModelRequest>) -> impl Responder {
    let constructed_body = serde_json::json!({ "input_text": req_body.input_text });
    // Rest of the existing function remains the same
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

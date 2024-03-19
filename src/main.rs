async fn language_model_handler(req_body: web::Json<LanguageModelRequest>) -> impl Responder {
    let constructed_body = serde_json::json!({ "input_text": req_body.input_text });
    // Rest of the existing function remains the same
}

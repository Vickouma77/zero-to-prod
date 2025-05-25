use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct BodyData {
    _title: String,
    _content: Content,
}

#[derive(serde::Deserialize)]
pub struct Content {
    _text: String,
    _html: String,
}

pub async fn publish_newsletter(_body: web::Json<BodyData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

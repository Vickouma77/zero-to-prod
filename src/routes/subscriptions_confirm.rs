use actix_web::HttpResponse;

#[tracing::instrument(name = "Confirming a pending subscription")]
pub async fn confirm() -> HttpResponse {
    HttpResponse::Ok().finish()
}

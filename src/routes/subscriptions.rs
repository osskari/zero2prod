use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    _email: String,
    _name: String,
}

pub async fn subscribe(_body: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

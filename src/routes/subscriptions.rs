use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_body: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
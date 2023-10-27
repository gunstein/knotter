use actix_web::{HttpResponse, Responder};
use actix_web::get;
use crate::domain::dtos::health_response_dto::HealthResponse;

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = HealthResponse {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}
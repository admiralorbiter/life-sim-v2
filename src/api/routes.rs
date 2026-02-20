use actix_web::{web, HttpResponse, Responder};

/// Health check / placeholder endpoint.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "game": "Life Roguelite",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Configure all API routes.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(health))
    );
}

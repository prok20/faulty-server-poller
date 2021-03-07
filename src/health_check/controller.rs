use actix_web::{web, HttpResponse, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health_check", web::get().to(health_check));
}

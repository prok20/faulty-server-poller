use actix_web::{guard, web, HttpResponse};

use crate::polling::ports::{PollingService, RunId, StartRunRequestDto};

async fn start_run<T: PollingService>(
    _service: web::Data<T>,
    _request_payload: web::Json<StartRunRequestDto>,
) -> HttpResponse {
    unimplemented!()
}

async fn get_run<T: PollingService>(_service: web::Data<T>, _id: web::Path<RunId>) -> HttpResponse {
    unimplemented!()
}

pub fn configure<T: 'static + PollingService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route(
        "/runs",
        web::post()
            .guard(guard::Header("Content-Type", "application/json"))
            .to(start_run::<T>),
    );
    cfg.route("/runs/{id}", web::get().to(get_run::<T>));
}

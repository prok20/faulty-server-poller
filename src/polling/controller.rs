use actix_web::{web, HttpResponse};

use crate::polling::ports::{PollingService, RunId};

async fn start_run<T: PollingService>(_service: web::Data<T>) -> HttpResponse {
    unimplemented!()
}

async fn get_run<T: PollingService>(_service: web::Data<T>, _id: web::Path<RunId>) -> HttpResponse {
    unimplemented!()
}

pub fn configure<T: 'static + PollingService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/runs", web::post().to(start_run::<T>));
    cfg.route("/runs/{id}", web::get().to(get_run::<T>));
}

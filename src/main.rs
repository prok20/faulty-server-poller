use actix_web::{web, App, HttpServer};
use faulty_server_poller::configuration::get_settings;

#[actix_web::main]
async fn main() {
    run_app().await;
}

async fn run_app() {
    let settings = get_settings().expect("Failed to get configuration");

    HttpServer::new(move || App::new().configure(|cfg| configure_features(cfg)))
        .bind(settings.application.address())
        .expect("Unable to bind server to an address")
        .run()
        .await
        .expect("Failed to run the server");

    println!("{:?}", settings);
}

fn configure_features(cfg: &mut web::ServiceConfig) {
    configure_health_check(cfg);
    configure_poller(cfg);
}

fn configure_health_check(cfg: &mut web::ServiceConfig) {
    use faulty_server_poller::health_check::controller;

    controller::configure(cfg);
}

fn configure_poller(_cfg: &mut web::ServiceConfig) {
    unimplemented!()
}

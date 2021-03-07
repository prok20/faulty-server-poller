use actix_web::{web, App, HttpServer};

#[allow(unreachable_code)]
#[actix_web::main]
async fn main() {
    HttpServer::new(move || App::new().configure(|cfg| configure_features(cfg)))
        .bind("localhost:8080") // TODO: replace with address gained from settings
        .expect("Unable to bind server to an address")
        .run()
        .await
        .expect("Failed to run the server");
}

fn configure_features(cfg: &mut web::ServiceConfig) {
    configure_health_check(cfg);
    configure_poller(cfg);
}

fn configure_health_check(cfg: &mut web::ServiceConfig) {
    use faulty_server_poller::health_check::controller;

    controller::configure(cfg);
}

#[allow(unreachable_code)]
fn configure_poller(_cfg: &mut web::ServiceConfig) {
    unimplemented!()
}

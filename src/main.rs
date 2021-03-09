use actix_web::{web, App, HttpServer};
use faulty_server_poller::configuration::get_settings;
use faulty_server_poller::configuration::settings::Settings;
use faulty_server_poller::polling::background_job_runner::TokioBackgroundJobRunner;
use faulty_server_poller::polling::polling_service::{PollingService, PollingServiceImpl};
use faulty_server_poller::polling::request_sender::ReqwestRequestSender;
use faulty_server_poller::polling::run_repository::PostgresRunRepository;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() {
    run_app().await;
}

async fn run_app() {
    let settings = get_settings().expect("Failed to get configuration");
    let polling_service = build_polling_service(&settings).await;

    HttpServer::new(move || {
        App::new().configure(|cfg| {
            configure_health_check(cfg);
            configure_poller(cfg, polling_service.clone());
        })
    })
    .bind(settings.application.address())
    .expect("Unable to bind server to an address")
    .run()
    .await
    .expect("Failed to run the server");
}

fn configure_health_check(cfg: &mut web::ServiceConfig) {
    use faulty_server_poller::health_check::controller;

    controller::configure(cfg);
}

fn configure_poller(cfg: &mut web::ServiceConfig, service: impl PollingService + 'static) {
    use faulty_server_poller::polling::controller;

    let service = web::Data::new(service);

    controller::configure(service, cfg);
}

type PollingServiceType = PollingServiceImpl<
    PostgresRunRepository,
    TokioBackgroundJobRunner<PostgresRunRepository, ReqwestRequestSender>,
>;

async fn build_polling_service(settings: &Settings) -> PollingServiceType {
    let db_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(
            settings.database.connect_timeout_sec,
        ))
        .connect_with(settings.database.connection_options())
        .await
        .expect("Failed to connect to database");
    let run_repo = PostgresRunRepository::new(db_pool);

    let request_sender = ReqwestRequestSender::new(
        reqwest::Client::new(),
        settings.polling.polling_address.clone(),
    );

    let job_runner =
        TokioBackgroundJobRunner::new(run_repo.clone(), request_sender, settings.polling.clone())
            .await;

    PollingServiceImpl::new(run_repo, job_runner)
}

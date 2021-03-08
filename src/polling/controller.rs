use actix_web::{guard, web, Responder};

use crate::polling::dto::{RunId, StartRunRequestDto};
use crate::polling::errors::ServiceResult;
use crate::polling::polling_service::PollingService;

async fn start_run<T: PollingService>(
    service: web::Data<T>,
    request_payload: web::Json<StartRunRequestDto>,
) -> ServiceResult<impl Responder> {
    service
        .start_run(request_payload.into_inner())
        .await
        .map(web::Json)
}

async fn get_run<T: PollingService>(
    service: web::Data<T>,
    id: web::Path<RunId>,
) -> ServiceResult<impl Responder> {
    service.get_run(id.into_inner()).await.map(web::Json)
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

#[cfg(test)]
mod should {
    use super::*;
    use crate::polling::dto::{Run, RunStatus, StartRunResponseDto};
    use crate::polling::polling_service::MockPollingService;
    use actix_web::{test, App};
    use mockall::predicate::*;
    use std::str::FromStr;

    #[actix_rt::test]
    async fn start_new_run() {
        let request_payload = StartRunRequestDto { seconds: 30 };
        let expected_response = StartRunResponseDto {
            id: RunId::from_str("cdc97318-ffb2-4350-b8d9-446cdd773a08").unwrap(),
        };

        let polling_service = {
            let mut ps = MockPollingService::new();
            ps.expect_start_run()
                .with(eq(request_payload.clone()))
                .return_const(Ok(expected_response.clone()));
            web::Data::new(ps)
        };

        let app =
            test::init_service(App::new().configure(|cfg| configure(polling_service, cfg))).await;

        let request = test::TestRequest::post()
            .uri("/runs")
            .set_json(&request_payload)
            .to_request();

        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());

        let actual_response: StartRunResponseDto = test::read_body_json(response).await;
        assert_eq!(expected_response, actual_response);
    }

    #[actix_rt::test]
    async fn get_existing_run() {
        let run_id = RunId::from_str("cdc97318-ffb2-4350-b8d9-446cdd773a08").unwrap();
        let expected_response = Run {
            id: run_id,
            status: RunStatus::Finished,
            successful_responses_count: 10,
            sum: 300,
        };

        let polling_service = {
            let mut ps = MockPollingService::new();
            ps.expect_get_run()
                .with(eq(run_id))
                .return_const(Ok(expected_response.clone()));
            web::Data::new(ps)
        };

        let app =
            test::init_service(App::new().configure(|cfg| configure(polling_service, cfg))).await;

        let request = test::TestRequest::get()
            .uri(&*format!("/runs/{}", run_id))
            .to_request();

        let response = test::call_service(&app, request).await;

        assert!(response.status().is_success());

        let actual_response: Run = test::read_body_json(response).await;
        assert_eq!(expected_response, actual_response);
    }
}

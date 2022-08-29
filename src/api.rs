use actix_web::{web, HttpResponse, Responder};
use platz_sdk::{PlatzStatus, Status, StatusColor};
use serde::Serialize;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/status", web::get().to(status));
}

#[derive(Clone, Serialize)]
enum FarmStatus {
    Open,
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(PlatzStatus {
        status: Status {
            name: FarmStatus::Open,
            color: StatusColor::Success,
        },
        primary_metric: None,
        metrics: None,
        notices: Default::default(),
    })
}

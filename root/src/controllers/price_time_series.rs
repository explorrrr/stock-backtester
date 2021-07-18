use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::config::setting::RepositoriesImpls;
use crate::domain::service::price_time_series::PriceTimeSeriesDomainService;

#[derive(Deserialize, Debug)]
pub struct PriceSeriesRequest {
    pub time_stamp: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(Deserialize, Debug)]
pub struct PriceTimeSeriesStoreRequest {
    pub ticker: String,
    pub price_time_series_data: Vec<PriceSeriesRequest>,
}

#[derive(Serialize, Debug)]
struct TimeSeriesStoreResponse {
    msg: String,
}

#[post("/timeseries")]
async fn store_price_time_series_data(
    request_body: web::Json<PriceTimeSeriesStoreRequest>,
) -> Result<HttpResponse, Error> {
    let repo = RepositoriesImpls::default();
    let price_time_series_domain_service = PriceTimeSeriesDomainService::new(&repo);

    let _ = price_time_series_domain_service
        .store_time_series_data(request_body.0)
        .await;

    let msg = "Store time series data successfully".to_string();
    Ok(HttpResponse::Ok().json(TimeSeriesStoreResponse { msg }))
}

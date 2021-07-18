use actix_web::{post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::config::setting::RepositoriesImpls;
use crate::domain::service::price_time_series::{BackTestResult, PriceTimeSeriesDomainService};

#[derive(Deserialize, Debug)]
pub struct OrderRequest {
    pub time_stamp: String,
    pub amount: f64,
    pub side: String,
}

#[derive(Deserialize, Debug)]
pub struct BackTestRequest {
    pub initial_asset: f64,
    pub ticker: String,
    pub orders: Vec<OrderRequest>,
}

#[derive(Serialize, Debug)]
struct BackTestResponse {
    test_result: BackTestResult,
    msg: String,
}

#[post("/backtest")]
async fn back_test(request_body: web::Json<BackTestRequest>) -> Result<HttpResponse, Error> {
    let repo = RepositoriesImpls::default();
    let price_time_series_domain_service = PriceTimeSeriesDomainService::new(&repo);

    let result = price_time_series_domain_service
        .backtest(request_body.0)
        .await;

    let msg = "Store time series data successfully".to_string();
    Ok(HttpResponse::Ok().json(BackTestResponse {
        test_result: result,
        msg,
    }))
}

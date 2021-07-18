use chrono::{TimeZone, Utc};
use serde::Serialize;

use crate::controllers::back_test::BackTestRequest;
use crate::controllers::price_time_series::PriceTimeSeriesStoreRequest;
use crate::domain::entity::price_time_series::{PriceSeriesEntity, PriceTimeSeriesEntity};
use crate::domain::repository::price_time_series::IPriceTimeSeriesRepository;
use crate::domain::repository::repositories::Repositories;

#[derive(Serialize, Debug)]
pub struct BackTestResult {
    result: Vec<DayResult>,
}

#[derive(Serialize, Debug)]
pub struct DayResult {
    asset: f64,
    holding_long_positions: f64,
    holding_short_positions: f64,
}

pub struct PriceTimeSeriesDomainService<'r, R: Repositories> {
    price_time_series_repository: &'r R::PriceTimeSeriesRepo,
}

impl<'r, R: Repositories> PriceTimeSeriesDomainService<'r, R>
where
    <R as Repositories>::PriceTimeSeriesRepo: Sync,
{
    pub fn new(repositories: &'r R) -> Self {
        Self {
            price_time_series_repository: repositories.price_time_series_repository(),
        }
    }

    pub async fn store_time_series_data(&self, request: PriceTimeSeriesStoreRequest) {
        let mut price_vec: Vec<PriceSeriesEntity> = vec![];
        let start_datetime_str = &request.price_time_series_data.first().unwrap().time_stamp;
        let end_datetime_str = &request.price_time_series_data.last().unwrap().time_stamp;

        for price in &request.price_time_series_data {
            let price_entity = PriceSeriesEntity::new(
                &price.time_stamp,
                price.open,
                price.close,
                price.high,
                price.low,
            );
            price_vec.push(price_entity)
        }

        let price_meta_entity = PriceTimeSeriesEntity::new(
            &request.ticker,
            start_datetime_str,
            end_datetime_str,
            price_vec,
        );

        let is_duplicated = self
            .price_time_series_repository
            .is_duplicated(&request.ticker, start_datetime_str, end_datetime_str)
            .await;

        if !is_duplicated {
            self.price_time_series_repository
                .create(price_meta_entity)
                .await;
        }
    }

    pub async fn backtest(&self, request: BackTestRequest) -> BackTestResult {
        let time_series_entity = self
            .price_time_series_repository
            .get_time_series_by_ticker(&request.ticker)
            .await;

        let start_time = Utc
            .datetime_from_str(&request.orders[0].time_stamp, "%Y/%m/%d %H:%M:%S")
            .unwrap();

        let mut day_result_vec: Vec<DayResult> = vec![];
        let mut net_position = 0.0;
        let mut asset = request.initial_asset;
        let mut test_time_series: Vec<PriceSeriesEntity> = vec![];

        for entity in time_series_entity {
            if entity.time_stamp >= start_time {
                test_time_series.push(entity)
            }
        }

        for order in request.orders {
            if order.side == "BUY" {
                net_position += order.amount;
            } else if order.side == "SELL" {
                net_position -= order.amount
            }

            let mut holding_long_positions = 0.0;
            let mut holding_short_positions = 0.0;

            let order_datetime = Utc
                .datetime_from_str(&order.time_stamp, "%Y/%m/%d %H:%M:%S")
                .unwrap();

            // FIXME bad implementation, fix this later.
            for test_data in &test_time_series {
                if test_data.time_stamp == order_datetime {
                    if net_position > 0.0 {
                        holding_long_positions = net_position.abs();
                    } else if net_position < 0.0 {
                        holding_short_positions = net_position.abs();
                    }
                    asset += (test_data.close_price - test_data.open_price) * net_position;

                    let day_result = DayResult {
                        asset,
                        holding_long_positions,
                        holding_short_positions,
                    };
                    day_result_vec.push(day_result);
                    break;
                }
            }
        }
        BackTestResult {
            result: day_result_vec,
        }
    }
}

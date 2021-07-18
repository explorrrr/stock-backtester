use async_trait::async_trait;

use crate::domain::entity::price_time_series::PriceTimeSeriesEntity;
use crate::domain::entity::price_time_series::PriceSeriesEntity;


#[async_trait]
pub trait IPriceTimeSeriesRepository {
    async fn is_duplicated(&self, _input_ticker: &str, _start_datetime_str: &str, _end_datetime_str: &str) -> bool {
        unimplemented!()
    }

    async fn create(&self, _entity: PriceTimeSeriesEntity) {
        unimplemented!()
    }

    async fn get_time_series_by_ticker(&self, _input_ticker: &str) -> Vec<PriceSeriesEntity> {
        unimplemented!()
    }
}

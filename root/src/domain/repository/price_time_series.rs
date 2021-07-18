use async_trait::async_trait;

use crate::domain::entity::price_time_series::PriceTimeSeriesEntity;


#[async_trait]
pub trait IPriceTimeSeriesRepository {
    async fn is_duplicated(&self, _ticker: &str, _start_datetime_str: &str, _end_datetime_str: &str) -> bool {
        unimplemented!()
    }
    async fn create(&self, _entity: PriceTimeSeriesEntity) {
        unimplemented!()
    }
}

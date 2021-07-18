use crate::controllers::price_time_series::PriceTimeSeriesStoreRequest;
use crate::domain::entity::price_time_series::{PriceSeriesEntity, PriceTimeSeriesEntity};
use crate::domain::repository::price_time_series::IPriceTimeSeriesRepository;
use crate::domain::repository::repositories::Repositories;

pub struct PriceTimeSeriesDomainService<'r, R: Repositories> {
    price_time_series_repository: &'r R::PriceTimeSeriesRepo
}

impl <'r, R: Repositories> PriceTimeSeriesDomainService<'r, R>
where
    <R as Repositories>::PriceTimeSeriesRepo: Sync,
{
    pub fn new(repositories: &'r R) -> Self {
        Self {
            price_time_series_repository: repositories.price_time_series_repository()
        }
    }

    pub async fn store_time_series_data(
        &self,
        request: PriceTimeSeriesStoreRequest
    ) {

        let mut price_vec: Vec<PriceSeriesEntity> = vec![];
        let start_datetime_str = &request.price_time_series_data.first().unwrap().time_stamp;
        let end_datetime_str = &request.price_time_series_data.last().unwrap().time_stamp;

        for price in &request.price_time_series_data {
            let price_entity = PriceSeriesEntity::new(&price.time_stamp, price.open, price.close, price.high, price.low);
            price_vec.push(price_entity)
        }

        let price_meta_entity = PriceTimeSeriesEntity::new(&request.ticker, start_datetime_str, end_datetime_str, price_vec);

        let is_duplicated = self.price_time_series_repository.is_duplicated(&request.ticker, start_datetime_str, end_datetime_str).await;

        if !is_duplicated {
            self.price_time_series_repository.create(price_meta_entity).await;
        }
    }
}
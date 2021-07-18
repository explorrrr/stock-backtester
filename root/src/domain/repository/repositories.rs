use crate::domain::repository::price_time_series::IPriceTimeSeriesRepository;

pub trait Repositories {
    // time series
    type PriceTimeSeriesRepo: IPriceTimeSeriesRepository;
    fn price_time_series_repository(&self) -> &Self::PriceTimeSeriesRepo;
}

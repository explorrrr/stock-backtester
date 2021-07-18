use envy;
use serde::Deserialize;
use std::process;

use crate::domain::repository::repositories::Repositories;
use crate::infrastructure::repository::price_time_series::PriceTimeSeriesPostgresRepository;

#[derive(Clone, Default)]
pub struct RepositoriesImpls {
    // time series
    price_time_series_repository: PriceTimeSeriesPostgresRepository,
}

impl Repositories for RepositoriesImpls {
    // time series
    type PriceTimeSeriesRepo = PriceTimeSeriesPostgresRepository;
    fn price_time_series_repository(&self) -> &Self::PriceTimeSeriesRepo {
        &self.price_time_series_repository
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
}

pub fn get_config() -> Config {
    // 設定項目の取得
    let config: Config = match envy::from_env::<Config>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    config
}

use async_trait::async_trait;
use diesel::prelude::*;

use crate::domain::repository::price_time_series::IPriceTimeSeriesRepository;
use crate::infrastructure::lib::postgres::establish_connection;

use crate::domain::entity::price_time_series::{PriceSeriesEntity, PriceTimeSeriesEntity};
use crate::infrastructure::models::price_time_series::{
    NewPriceTimeSeries, NewPriceTimeSeriesMeta, PriceTimeSeries, PriceTimeSeriesMeta,
};
use crate::schema::price_time_series;
use crate::schema::price_time_series::dsl::*;
use crate::schema::price_time_series_meta;
use crate::schema::price_time_series_meta::dsl::*;

#[derive(Clone)]
pub struct PriceTimeSeriesPostgresRepository {}

impl Default for PriceTimeSeriesPostgresRepository {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl IPriceTimeSeriesRepository for PriceTimeSeriesPostgresRepository {
    async fn is_duplicated(
        &self,
        input_ticker: &str,
        _start_datetime_str: &str,
        _end_datetime_str: &str,
    ) -> bool {
        let conn = establish_connection();

        let result = price_time_series_meta
            .filter(
                price_time_series_meta::is_deleted
                    .eq(false)
                    .and(ticker.eq(input_ticker)),
            )
            .load::<PriceTimeSeriesMeta>(&conn)
            .expect("Error loading");

        return !result.is_empty();
    }

    async fn create(&self, entity: PriceTimeSeriesEntity) {
        let conn = establish_connection();

        let new_price_time_series_meta = NewPriceTimeSeriesMeta {
            id: &entity.id,
            ticker: &entity.ticker,
        };

        let _ = diesel::insert_into(price_time_series_meta::table)
            .values(&new_price_time_series_meta)
            .get_result::<PriceTimeSeriesMeta>(&conn)
            .expect("Error");

        for price in entity.series {
            let new_price_time_series = NewPriceTimeSeries {
                id: &price.id,
                price_time_series_meta_id: &entity.id,
                time_stamp: &price.time_stamp,
                open_price: &price.open_price,
                close_price: &price.close_price,
                high_price: &price.high_price,
                low_price: &price.low_price,
            };

            // FIXME for insert because of lifetime
            let _ = diesel::insert_into(price_time_series::table)
                .values(&new_price_time_series)
                .get_result::<PriceTimeSeries>(&conn)
                .expect("Error");
        }
    }

    async fn get_time_series_by_ticker(&self, input_ticker: &str) -> Vec<PriceSeriesEntity> {
        let mut price_time_series_vec: Vec<PriceSeriesEntity> = vec![];
        let conn = establish_connection();

        let result_meta = price_time_series_meta
            .filter(
                price_time_series_meta::is_deleted
                    .eq(false)
                    .and(ticker.eq(input_ticker)),
            )
            .first::<PriceTimeSeriesMeta>(&conn)
            .expect("Error loading");

        let result = price_time_series
            .filter(
                price_time_series::is_deleted
                    .eq(false)
                    .and(price_time_series::price_time_series_meta_id.eq(result_meta.id)),
            )
            .load::<PriceTimeSeries>(&conn)
            .expect("Error loading");

        for price in result {
            let entity = PriceSeriesEntity {
                id: price.id,
                time_stamp: price.time_stamp,
                open_price: price.open_price,
                close_price: price.close_price,
                high_price: price.high_price,
                low_price: price.low_price,
            };

            price_time_series_vec.push(entity)
        }

        price_time_series_vec
    }
}

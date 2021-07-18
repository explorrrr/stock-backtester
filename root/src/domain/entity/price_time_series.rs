use uuid::Uuid;
use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct PriceSeriesEntity {
    pub id: String,
    pub time_stamp: DateTime<Utc>,
    pub open_price: f64,
    pub close_price: f64,
    pub high_price: f64,
    pub low_price: f64
}

impl PriceSeriesEntity {
    pub fn new(
        datetime_str: &str,
        open_price: f64,
        close_price: f64,
        high_price: f64,
        low_price: f64,
    ) -> Self {

        let id = Uuid::new_v4().to_string();
        let datetime: DateTime<Utc> = Utc.datetime_from_str(datetime_str, "%Y/%m/%d %H:%M:%S").unwrap();

        Self {
            id,
            time_stamp: datetime,
            open_price,
            close_price,
            high_price,
            low_price
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct PriceTimeSeriesEntity {
    pub id: String,
    pub ticker: String,
    pub start_datetime: DateTime<Utc>,
    pub end_datetime: DateTime<Utc>,
    pub series: Vec<PriceSeriesEntity>
}

impl PriceTimeSeriesEntity {
    pub fn new(
        ticker: &str,
        start_datetime_str: &str,
        end_datetime_str: &str,
        price_time_series_data: Vec<PriceSeriesEntity>
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let start_datetime: DateTime<Utc> = Utc.datetime_from_str(start_datetime_str, "%Y/%m/%d %H:%M:%S").unwrap();
        let end_datetime: DateTime<Utc> = Utc.datetime_from_str(end_datetime_str, "%Y/%m/%d %H:%M:%S").unwrap();

        Self {
            id,
            ticker: ticker.to_string(),
            start_datetime,
            end_datetime,
            series: price_time_series_data
        }
    }
}

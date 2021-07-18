use chrono::{DateTime, Utc};

use crate::schema::price_time_series_meta;
use crate::schema::price_time_series;


#[derive(Insertable)]
#[table_name = "price_time_series_meta"]
pub struct NewPriceTimeSeriesMeta<'a> {
    pub id: &'a str,
    pub ticker: &'a str,
}

#[derive(Insertable)]
#[table_name = "price_time_series"]
pub struct NewPriceTimeSeries<'a> {
    pub id: &'a str,
    pub price_time_series_meta_id: &'a str,
    pub time_stamp: &'a DateTime<Utc>,
    pub open_price: &'a f64,
    pub close_price: &'a f64,
    pub high_price: &'a f64,
    pub low_price: &'a f64,
}

#[derive(Queryable, Debug)]
pub struct PriceTimeSeriesMeta {
    pub id: String,
    pub ticker: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool
}

#[derive(Queryable, Debug)]
pub struct PriceTimeSeries {
    pub id: String,
    pub price_time_series_meta_id: String,
    pub time_stamp: DateTime<Utc>,
    pub open_price: f64,
    pub close_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool
}

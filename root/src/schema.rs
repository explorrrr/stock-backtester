table! {
    price_time_series (id) {
        id -> Varchar,
        price_time_series_meta_id -> Varchar,
        time_stamp -> Timestamptz,
        open_price -> Float8,
        close_price -> Float8,
        high_price -> Float8,
        low_price -> Float8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        is_deleted -> Bool,
    }
}

table! {
    price_time_series_meta (id) {
        id -> Varchar,
        ticker -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
        is_deleted -> Bool,
    }
}

joinable!(price_time_series -> price_time_series_meta (price_time_series_meta_id));

allow_tables_to_appear_in_same_query!(
    price_time_series,
    price_time_series_meta,
);

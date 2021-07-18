CREATE TABLE if NOT exists price_time_series_meta
(
    id VARCHAR(64) NOT NULL constraint price_time_series_meta_pkey PRIMARY KEY,
    ticker VARCHAR(64) NOT NULL ,
    created_at     TIMESTAMP with TIME zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP with TIME zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at     TIMESTAMP with TIME zone,
    is_deleted boolean DEFAULT false NOT NULL
);

CREATE TABLE if NOT exists price_time_series
(
    id VARCHAR(64) NOT NULL constraint price_time_series_pkey PRIMARY KEY,
    price_time_series_meta_id VARCHAR(64) NOT NULL constraint price_time_series_meta_id_fkey REFERENCES price_time_series_meta,
    time_stamp TIMESTAMP with TIME zone NOT NULL ,
    open_price FLOAT NOT NULL ,
    close_price FLOAT NOT NULL ,
    high_price FLOAT NOT NULL ,
    low_price FLOAT NOT NULL ,
    created_at     TIMESTAMP with TIME zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at     TIMESTAMP with TIME zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    deleted_at     TIMESTAMP with TIME zone,
    is_deleted boolean DEFAULT false NOT NULL
);

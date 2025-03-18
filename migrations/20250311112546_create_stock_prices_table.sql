create extension if not exists timescaledb;

create table stock_prices (
    time            timestamptz         not null,
    stock_id        uuid                not null references stocks(id),
    price           numeric             not null,
    primary key(time, stock_id)
);

select create_hypertable('stock_prices', 'time');

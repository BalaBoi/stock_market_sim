create extension if not exists timescaledb;

create table stock_prices (
    time            timestamptz         not null,
    stock_id        uuid                not null references stocks(id),
    price           numeric             not null,
    primary key(time, stock_id)
);

select create_hypertable('stock_prices', 'time');

create or replace function update_stock_price()
returns trigger as $$
begin
    insert into stock_prices (time, stock_id, price)
    values (NEW.updated_at, NEW.id, NEW.current_price);
    
    return NEW;
end;
$$ language plpgsql;

create trigger sync_stock_price_trigger
after insert or update of current_price on stocks
for each row execute function update_stock_price();
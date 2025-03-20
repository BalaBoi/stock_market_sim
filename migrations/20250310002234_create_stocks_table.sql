create table stocks (
    id              uuid            primary key default uuid_generate_v1mc(),
    symbol          varchar(10)     unique not null collate "case_insensitive",
    name            text            not null,
    current_price   numeric         not null,
    created_at      timestamptz     not null default now(),
    updated_at      timestamptz     not null default now()
);

select trigger_updated_at('stocks');

create or replace function enforce_uppercase_symbols()
returns trigger as $$
begin
    new.symbol = upper(new.symbol);
    return new;
end;
$$ language plpgsql;

create trigger uppercase_symbol_trigger
before insert or update on stocks
for each row execute function enforce_uppercase_symbols();

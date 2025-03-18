create table users (
    id              uuid            primary key default uuid_generate_v1mc(),
    username        varchar(50)     unique not null collate case_insensitive,
    email           varchar(255)    unique not null collate case_insensitive,
    password_hash   text            not null,
    cash_balance    numeric         not null default 0,
    created_at      timestamptz     not null default now(),
    updated_at      timestamptz     not null default now()
);

select trigger_updated_at('users');

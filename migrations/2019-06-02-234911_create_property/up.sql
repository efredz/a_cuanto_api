-- Your SQL goes here
CREATE table property(
    id serial primary key,
    general_location varchar not null,
    price integer  not null,
    published varchar not null,
    uri varchar not null
)
-- Your SQL goes here
create table users(
    id serial primary key,
    username varchar(255) not null unique,
    email varchar(300) not null unique,
    password_hash varchar(356) not null
);
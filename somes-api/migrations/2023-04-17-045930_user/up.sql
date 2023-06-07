-- Your SQL goes here
create table users(
    id serial unique,
    username varchar(255) not null primary key,
    email varchar(300) not null unique,
    password_hash varchar(356) not null
);
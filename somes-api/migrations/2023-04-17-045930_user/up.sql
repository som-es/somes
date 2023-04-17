-- Your SQL goes here
create table users(
    id serial primary key,
    username varchar(255),
    email varchar(300),
    password_hash varchar(356)
);
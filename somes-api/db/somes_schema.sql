create table user (
    id integer primary key AUTOINCREMENT,
    username varchar(120),
    email varchar(300),
    password_hash varchar(300)
);
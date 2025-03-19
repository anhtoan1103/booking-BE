drop table if exists users;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL  -- Hashed password
);


insert into users(email, password) values('toto@gmail.com', 'hehe');
insert into users(email, password, id) values('toto@gmail.com', 'hehe', 1);


create table if not exists users
(
    id                serial                    not null primary key,
    email             text                      not null unique,
    password          text                      not null,
    first_name        text                      not null,
    last_name         text                      not null,
    created_at        timestamptz default now() not null,
    updated_at        timestamptz default now() not null
);

create table if not exists role
(
    id   integer not null primary key,
    role text    not null unique
);

create table if not exists user_role
(
    user_id integer not null references users (id),
    role_id integer not null references role (id)
);

-- @formatter:off
insert into role values (1, 'User');
insert into role values (2, 'Admin');

-- Default client
-- Anonymous user
insert into users (id, email, password, first_name, last_name) values
    ((select nextval('users_id_seq')), '', '', 'Anonymous', '');

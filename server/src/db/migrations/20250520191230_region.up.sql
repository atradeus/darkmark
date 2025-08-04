create table if not exists region
(
    id         serial                   not null primary key,
    code       text                      not null,
    name       text                      not null,
    geography  text                      not null,
    created_at timestamptz default now() not null,
    updated_at timestamptz default now() not null
);

insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'us-east-1',
        'US East (N. Virginia)',
        'USA');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'us-east-2',
        'US East (Ohio)',
        'USA');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'us-west-1',
        'US West (N. California)',
        'USA');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'us-west-2',
        'US West (Oregon)',
        'USA');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'eu-central-1',
        'Europe (Frankfurt)',
        'Germany');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'eu-west-1',
        'Europe (Ireland)',
        'Ireland');
insert into region (id, code, name, geography)
values ((select nextval('region_id_seq')),
        'eu-west-2',
        'Europe (London)',
        'United Kingdom');



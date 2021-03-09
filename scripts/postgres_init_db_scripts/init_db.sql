create user faulty_server_poller_service with password 'service_password';

create schema faulty_server_poller;
grant usage on schema faulty_server_poller to faulty_server_poller_service;

alter role postgres set search_path to faulty_server_poller,public;
alter role faulty_server_poller_service set search_path to faulty_server_poller;

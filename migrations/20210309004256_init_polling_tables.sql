create table run_status
(
    status_id   smallint,
    status_name varchar(256) not null,
    primary key (status_id)
);

create table run
(
    run_id                   uuid,
    status_id                smallint  not null default 0,
    run_insertion_datetime   timestamp not null default localtimestamp,
    run_successful_responses bigint not null default 0,
    run_value_sum            bigint not null default 0,
    primary key (run_id),
    constraint fk_status
        foreign key (status_id)
            references run_status (status_id)
);

insert into run_status (status_id, status_name)
values (0, 'IN_PROGRESS'),
       (1, 'FINISHED');

grant select, insert, update on run to faulty_server_poller_service;
grant select on run_status to faulty_server_poller_service;

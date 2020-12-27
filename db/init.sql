CREATE TABLE events
(
    aggregate_type text                         NOT NULL,
    aggregate_id   text                         NOT NULL,
    sequence       bigint CHECK (sequence >= 0) NOT NULL,
    payload        jsonb                        NOT NULL,
    metadata       jsonb                        NOT NULL,
    timestamp      timestamp with time zone DEFAULT (CURRENT_TIMESTAMP),
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE ipam_query
(
    query_instance_id text                        NOT NULL,
    version           bigint CHECK (version >= 0) NOT NULL,
    payload           jsonb                       NOT NULL,
    PRIMARY KEY (query_instance_id)
);

CREATE USER ipam_user WITH ENCRYPTED PASSWORD 'secret_saucey';
GRANT ALL PRIVILEGES ON DATABASE postgres TO ipam_user;

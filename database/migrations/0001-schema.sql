create extension if not exists "uuid-ossp";

CREATE TABLE cookies
(
    id   int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    uuid uuid UNIQUE DEFAULT uuid_generate_v4()
);
INSERT INTO cookies(uuid) VALUES('77e6634e-428e-447f-ba5e-998f247995c8');

CREATE TABLE sessions
(
    id        int PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    uuid      uuid UNIQUE DEFAULT uuid_generate_v4() NOT NULL,
    cookie_id int                                    NOT NULL,
    size      text,
    clickpath text        DEFAULT ''                 NOT NULL,
    ip        text                                   NOT NULL,
    useragent text                                   NOT NULL,
    created   timestamptz DEFAULT now()              NOT NULL,

    CONSTRAINT fk_cookie
        FOREIGN KEY (cookie_id)
            REFERENCES cookies (id)
            ON DELETE CASCADE
);

CREATE TABLE limits
(
    ip         text        NOT NULL,
    counter    int         NOT NULL,
    last_taken timestamptz NOT NULL
);

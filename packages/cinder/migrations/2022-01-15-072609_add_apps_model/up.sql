CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE apps (
    id          SERIAL PRIMARY KEY,
    slug        VARCHAR NOT NULL DEFAULT uuid_generate_v4(),
    name        VARCHAR NOT NULL,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

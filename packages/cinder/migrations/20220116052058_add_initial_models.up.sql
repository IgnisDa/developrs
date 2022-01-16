-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "apps" (
    slug        UUID NOT NULL DEFAULT uuid_generate_v4(),
    name        TEXT PRIMARY KEY,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "deploys" (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    sha         VARCHAR NOT NULL UNIQUE,
    executed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    app_name    TEXT NOT NULL,

    CONSTRAINT fk_app FOREIGN KEY(app_name) REFERENCES apps(name) ON DELETE CASCADE ON UPDATE CASCADE
);

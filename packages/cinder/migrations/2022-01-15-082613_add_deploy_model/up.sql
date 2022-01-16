CREATE TABLE "deploys" (
    id          VARCHAR PRIMARY KEY DEFAULT uuid_generate_v4(),
    sha         VARCHAR NOT NULL,
    executed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    app_id      INT NOT NULL,

    CONSTRAINT fk_app FOREIGN KEY(app_id) REFERENCES apps(id) ON DELETE CASCADE
);

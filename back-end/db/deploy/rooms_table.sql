-- Deploy chat_app:rooms_table to pg

BEGIN;

CREATE TABLE IF NOT EXISTS rooms (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NULL,
    created_by BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    password_hash VARCHAR(255) NULL DEFAULT NULL,
    CONSTRAINT fk_rooms_users_id FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE CASCADE
);
COMMIT;

-- Revert chat_app:users_table from pg

BEGIN;

DROP TABLE IF EXISTS users;

COMMIT;

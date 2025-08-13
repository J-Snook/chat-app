-- Revert chat_app:rooms_table from pg

BEGIN;

DROP TABLE IF EXISTS rooms;

COMMIT;

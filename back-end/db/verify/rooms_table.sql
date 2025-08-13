-- Verify chat_app:rooms_table on pg

BEGIN;

SELECT id FROM rooms LIMIT 1;

ROLLBACK;

-- Verify chat_app:users_table on pg

BEGIN;

SELECT id FROM users LIMIT 1;

ROLLBACK;

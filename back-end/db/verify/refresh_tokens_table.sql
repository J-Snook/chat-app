-- Verify chat_app:refresh_tokens_table on pg

BEGIN;

SELECT id FROM refresh_tokens LIMIT 1;

ROLLBACK;

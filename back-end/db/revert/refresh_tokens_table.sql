-- Revert chat_app:refresh_tokens_table from pg

BEGIN;

DROP TABLE IF EXISTS refresh_tokens;

COMMIT;

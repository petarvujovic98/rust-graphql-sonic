-- This file should undo anything in `up.sql`
DROP INDEX items_text_tsv_index;
ALTER TABLE items DROP COLUMN text_tsv;

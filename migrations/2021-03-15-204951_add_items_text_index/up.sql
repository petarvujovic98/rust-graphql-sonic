-- Your SQL goes here
ALTER TABLE items ADD COLUMN text_tsv tsvector GENERATED ALWAYS AS (to_tsvector('english', text)) STORED;
CREATE INDEX items_text_tsv_index ON items USING GIN (text_tsv);

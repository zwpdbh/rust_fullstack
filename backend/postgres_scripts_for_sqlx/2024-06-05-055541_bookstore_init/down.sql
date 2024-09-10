-- This file should undo anything in `up.sql`
-- Drop all the indexes
DROP INDEX IF EXISTS idx_items_name;

DROP INDEX IF EXISTS idx_items_category_id;

DROP INDEX IF EXISTS book_isbn_idx;

-- Drop all the tables
DROP TABLE IF EXISTS orders;

DROP TABLE IF EXISTS items_taggings;

DROP TABLE IF EXISTS item_tags;

DROP TABLE IF EXISTS items;

DROP TABLE IF EXISTS categories;

DROP TABLE IF EXISTS todos;

DROP TABLE IF EXISTS book;
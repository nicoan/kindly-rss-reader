-- This table is to keep track of the applied migrations
CREATE TABLE IF NOT EXISTS migrations (
    -- name of the migrated file
    name TEXT PRIMARY KEY
);

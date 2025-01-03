CREATE TABLE IF NOT EXISTS feed (
    id VARCHAR(36) PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    favicon_path TEXT,
    last_updated DATE NOT NULL,

    UNIQUE(url)
);

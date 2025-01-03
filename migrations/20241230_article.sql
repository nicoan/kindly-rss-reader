CREATE TABLE IF NOT EXISTS article (
    id VARCHAR(36) PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT,
    description TEXT,
    guid TEXT,
    path TEXT,
    read SMALLINT,
    last_updated DATE NOT NULL,
    feed_id VARCHAR(16),
    FOREIGN KEY (feed_id) REFERENCES feed(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(guid)
);

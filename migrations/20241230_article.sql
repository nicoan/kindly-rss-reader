CREATE TABLE IF NOT EXISTS article (
    id VARCHAR(36) PRIMARY KEY,

    title TEXT NOT NULL,

    author TEXT,

    guid TEXT,
    -- The complete link to the article
    link TEXT,

    -- Content of the article. Depending on the storage engine used can be a fs path or the content itself
    -- For this implementation (SQLite) it is a path to the fs
    content TEXT,

    -- If the article  was read
    read SMALLINT,

    -- If the article  was exracted from an HTML instead of the content field in RSS
    html_parsed SMALLINT,


    -- This is the pub date. If for some reason the field is not available we put the date we parsed the article
    last_updated DATE NOT NULL,

    feed_id VARCHAR(16),

    FOREIGN KEY (feed_id) REFERENCES feed(id) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE(guid)
);

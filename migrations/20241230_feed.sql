-- SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
-- SPDX-License-Identifier: AGPL-3.0-only
CREATE TABLE IF NOT EXISTS feed (
    id VARCHAR(36) PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    link TEXT NOT NULL,
    favicon_path TEXT,
    last_updated DATE NOT NULL,

    UNIQUE(url)
);

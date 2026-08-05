-- SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
-- SPDX-License-Identifier: AGPL-3.0-only
-- This table is to keep track of the applied migrations
CREATE TABLE IF NOT EXISTS migrations (
    -- name of the migrated file
    name TEXT PRIMARY KEY
);

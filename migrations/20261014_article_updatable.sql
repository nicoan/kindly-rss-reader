-- SPDX-FileCopyrightText: 2025-2026 Nicolás Antinori <nico.antinori.7@gmail.com>
-- SPDX-License-Identifier: AGPL-3.0-only
ALTER TABLE article
ADD COLUMN updatable BOOLEAN NOT NULL DEFAULT 0;
